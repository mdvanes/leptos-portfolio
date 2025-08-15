use crate::api::{get_balance, AddTransactionToBalance};
use crate::utils::get_formatted_now;
use leptos::prelude::*;

#[component]
pub fn Balance(rate: Memo<Option<f64>>) -> impl IntoView {
    // let rate1 = rate.get().map(|rate| 1.0 * rate).unwrap_or(0.0); // Default to 0.0 if rate is None
    
    let add_transaction_to_balance = ServerAction::<AddTransactionToBalance>::new();

    // Create a resource that refetches when the action completes
    let balance_resource = Resource::new(
        move || add_transaction_to_balance.version().get(),
        |_| async move { get_balance().await },
    );

    view! {
        <div>
            <h2>"Current Balance"</h2>
            <Suspense fallback=move || view! { <p>"Loading balance..."</p> }>
                {move || {
                    balance_resource.get().map(|result| {
                        match result {
                            Ok(balance) => view! {
                                <div style="padding: 10px; background-color: #1a3448; border: 1px solid #4682b4; margin-bottom: 20px;">
                                    // TODO also show Balance in SYM based on current sym value
                                    <p style="font-size: 1.2em; font-weight: bold;">
                                        "Balance: €" {balance} " which is " 
                                        {rate.get().map(|r| balance / r).unwrap_or(0.0)} 
                                        " BTC"
                                    </p>
                                    // <p style="font-size: 1.2em; font-weight: bold;">"Balance: €" {balance} " which is " {rate1} " BTC"</p>
                                </div>
                            }.into_any(),
                            Err(err) => view! {
                                <div style="padding: 10px; background-color: #6e373f; border: 1px solid #f44336; margin-bottom: 20px;">
                                    <p style="color: red;">"Error loading balance: " {err.to_string()}</p>
                                </div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>

        <div>
            <h2>"Add a transaction"</h2>
            <ActionForm action=add_transaction_to_balance>
                <label for="number">"Amount deposited: € "</label>
                <input type="number" name="number" id="number" step="0.01" required />
                <input type="submit" value="Add" />
            </ActionForm>

            // Display the result of the action
            {move || {
                add_transaction_to_balance.value().get().map(|result| {
                    match result {
                        Ok(new_balance) => {
                            // Get current date/time using utility function
                            let time_str = get_formatted_now();

                            view! {
                                <div style="margin-top: 10px; padding: 10px; background-color: #4b7f4b; border: 1px solid #4caf50;">
                                    <p>"Transaction added at " {time_str} ". New balance: €" {new_balance}</p>
                                </div>
                            }.into_any()
                        },
                        Err(err) => view! {
                            <div style="margin-top: 10px; padding: 10px; background-color: #6e373f; border: 1px solid #f44336;">
                                <p style="color: red;">"Error: " {err.to_string()}</p>
                            </div>
                        }.into_any(),
                    }
                })
            }}
        </div>
    }
}
