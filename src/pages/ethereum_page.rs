use crate::api::get_rates;
use crate::components::{Balance, CurrencySymbol, Header, ServerTime};
use leptos::prelude::*;

#[component]
pub fn EthereumPage() -> impl IntoView {
    let rates_resource = LocalResource::new(|| async move { get_rates().await });

    let newRate = Memo::new(move |_| {
        rates_resource.get().and_then(|result| match result {
            Ok(rates_response) => rates_response.get(1).copied().map(|rate| rate / 1000.0),
            Err(_) => None,
        })
    });

    let currency_symbol = CurrencySymbol::ETH;

    view! {
        <Header/>
        <h1>"Ethereum Page"</h1>
        <p>"Welcome to the Ethereum information page!"</p>
        <div>
            <Suspense fallback=move || view! { <p>"Loading ETH rate..."</p> }>
                {move || {
                    rates_resource.get().map(|result| {
                        match result {
                            Ok(rates_response) => {
                                view! {
                                    <div>
                                        {rates_response.get(1).map(|rate| {
                                            view! {
                                                <p>"1 ETH is €" {format!("{:.2}", *rate / 1000.0)}</p>
                                            }
                                        })}
                                    </div>
                                }.into_any()
                            }
                            Err(err) => {
                                let error_msg = err.to_string();
                                view! {
                                    <div>
                                        <p style="color: red;">"Error loading ETH rate: " {error_msg}</p>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>

         <Balance rate=newRate currency_symbol=currency_symbol/>
         
         <ServerTime/>
    }
}
