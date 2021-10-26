use std::{borrow::BorrowMut, io};
use std::collections::{BinaryHeap};
use trading_lib::{Order, OrderType, construct_order};

pub fn is_loop_end_command(inp: &String) -> bool
{
    match inp.to_lowercase().trim() { // as_str doesn't match ?
        "end"   => true,
        "break" => true,
        "stop"  => true,
        ""      => true,
        _ => false
    }
}

fn main() 
{
    //binary heap as we want the lowest price first
    //and don't care about what it is otherwise
    let mut sales_queue:BinaryHeap<Order> = BinaryHeap::new();

    println!("Welcome to the bitcoin trading bot!");
    println!("Please enter an order with the form");
    println!("{{id}}: {{Buy|Sell}} {{quantity}} BTC @ {{price}}");
    println!("Type 'end', 'break', 'stop' or just enter to end the program.");
    println!("");
    
    loop 
    {
        println!("Your input:");
        let mut input = String::new();
        let user_in = io::stdin().read_line(&mut input);
        if user_in.is_err()
        {
            println!("error {}", user_in.err().unwrap());
            break;
        }

        if is_loop_end_command(&input)
        {
            println!("Ending ...");
            break;
        }
        // try to parse the input string to an order struct, continue loop 
        // if it does
        let mut o = match construct_order(&input) {
            Some(o) => o,
            None => {
                println!("\nERR: Couldn't parse input {{{}}}!", input);
                println!("\nPlease try again with the format");
                println!("{{id}}: {{Buy|Sell}} {{quantity}} BTC @ {{price}}");
                continue;
            }
        };


        if o.order_type == OrderType::Sell
        {   // we only care about saving sell orders as they can be used
            // for future buys if their quantity isn't depleted
            sales_queue.push(o);
        }
        else 
        {
            // keep going til the buy order is satisfied 
            // or no more sales are left
            while o.quantity > 0
            {
                let try_last_sell = sales_queue.pop();
                let mut last_sell = match try_last_sell {
                    Some(o) => {
                        if o.order_type == OrderType::Sell
                        {
                            o
                        }
                        else 
                        {//if no sales before we don't care
                            break;
                        }
                    },
                    None => { //queue is empty so abort
                        //println!("No more sales! Please input new orders ...");
                        //println!("Warn:[Removing last buy]");
                        break;
                    }
                };
                //this could be a string but we get more flexibility this way
                let t = trading_lib::do_trade(o.borrow_mut(), last_sell.borrow_mut());
                println!("{}", t.to_string());

                //this sale still has more BTC so push it back for use in the next buy
                if last_sell.quantity > 0
                {
                    sales_queue.push(last_sell);
                }
            }
        } //if
    } //loop
}

#[cfg(test)]
mod tests {
    use super::*;
    use trading_lib::{Trade, do_trade, lowest};

    // ------ Order TESTS ------
    #[test]
    fn check_order_equal() 
    {
        let o_a_1 = Order::new( 1, OrderType::Sell, 1, 1);
        let o_b_1 = Order::new( 1, OrderType::Sell, 1, 1);
        
        assert_eq!(o_a_1.id, o_b_1.id);
        assert_eq!(o_a_1.order_type, o_b_1.order_type);
        assert_eq!(o_a_1.price, o_b_1.price);
        assert_eq!(o_a_1.quantity, o_b_1.quantity);
        assert!(o_a_1.equal_to(&o_b_1));

        let o_a_2 = Order::new( 1, OrderType::Sell, 100, 1);
        let o_b_2 = Order::new( 1, OrderType::Sell, 1, 1);
        
        assert_eq!(o_a_2.id, o_b_2.id);
        assert_eq!(o_a_2.order_type, o_b_2.order_type);
        assert_ne!(o_a_2.price, o_b_2.price);
        assert_eq!(o_a_2.quantity, o_b_2.quantity);
        assert!(!o_a_2.equal_to(&o_b_2));

        let o_a_3 = Order::new( 1, OrderType::Sell, 1, 100);
        let o_b_3 = Order::new( 1, OrderType::Sell, 1, 1);
        
        assert_eq!(o_a_3.id, o_b_3.id);
        assert_eq!(o_a_3.order_type, o_b_3.order_type);
        assert_eq!(o_a_3.price, o_b_3.price);
        assert_ne!(o_a_3.quantity, o_b_3.quantity);
        assert!(!o_a_3.equal_to(&o_b_3));
    }
    #[test]
    fn construct_order_eq() 
    {
        let o = Order::new( 1, OrderType::Sell, 5000, 100);
        let o_const = trading_lib::construct_order(&String::from("1. Sell 100 BTC @ 5000 USD"));
        assert_ne!(None, o_const);
        assert_eq!(o, o_const.unwrap());
    }
    #[test]
    fn construct_empty_order_eq() 
    {
        let o = Order::new( 0, OrderType::Buy, 0, 0);
        assert_eq!(o, Order::empty());
    }
    #[test]
    fn construct_order_uneq() 
    {
        let o = Order::new( 13, OrderType::Buy, 50, 1000000);
        let o_const = trading_lib::construct_order(&String::from("1. Sell 100 BTC @ 5000 USD"));
        assert_ne!(None, o_const);
        assert_ne!(o, o_const.unwrap());
    }
    #[test]
    fn construct_order_not_enough_whitepspace() 
    {
        assert_eq!(None, trading_lib::construct_order(&String::from("ewughierguerguhrgi"))); //giberish
        assert_eq!(None, trading_lib::construct_order(&String::from("100 BTC 50 USD"))); //not enough info
    }

    
    // ------ Trade TESTS ------
    #[test]
    fn construct_trade_eq() 
    {
        let t = Trade::new(2,1,100, 100);
        let mut o_sell = Order::new( 1, OrderType::Sell, 100, 100);
        let mut o_buy = Order::new( 2, OrderType::Buy, 100, 100);
        assert_eq!(t, trading_lib::do_trade(o_buy.borrow_mut(), o_sell.borrow_mut()));
    }
    #[test]
    fn construct_trade_uneq() 
    {
        let t_1 = Trade::new(3,2,100, 100);
        let mut o_sell = Order::new( 1, OrderType::Sell, 100, 100);
        let mut o_buy = Order::new( 2, OrderType::Buy, 100, 100);
        assert_ne!(t_1, trading_lib::do_trade(o_buy.borrow_mut(), o_sell.borrow_mut()));

        let t_2 = Trade::new(1,2,100, 100);
        let mut o_sell = Order::new( 1, OrderType::Sell, 1000, 100);
        let mut o_buy = Order::new( 2, OrderType::Buy, 1000, 100);
        assert_ne!(t_2, trading_lib::do_trade(o_buy.borrow_mut(), o_sell.borrow_mut()));
        
        let t_3 = Trade::new(1,2,100, 100);
        let mut o_sell = Order::new( 1, OrderType::Sell, 100, 1000);
        let mut o_buy = Order::new( 2, OrderType::Buy, 100, 1000);
        assert_ne!(t_3, trading_lib::do_trade(o_buy.borrow_mut(), o_sell.borrow_mut()));
    }
    #[test]
    fn construct_trade_use_lowest_price() 
    {
        let mut o_sell_1 = Order::new( 1, OrderType::Sell, 1, 100);
        let mut o_buy_1 = Order::new( 2, OrderType::Buy, 2, 100);
        let t_1 = do_trade(o_buy_1.borrow_mut(), o_sell_1.borrow_mut());
        assert_eq!(t_1.price, 1);
        assert_eq!(t_1.price, o_sell_1.price);

        let mut o_sell_2 = Order::new( 1, OrderType::Sell, 2, 100);
        let mut o_buy_2 = Order::new( 2, OrderType::Buy, 1, 100);
        let t_2 = do_trade(o_buy_2.borrow_mut(),o_sell_2.borrow_mut());
        assert_eq!(t_2.price, 1);
        assert_eq!(t_2.price, o_buy_2.price);
    }
    #[test]
    fn check_b_heap_orders()
    {
        let mut bh:BinaryHeap<Order>= BinaryHeap::new();

        bh.push(Order::new( 1, OrderType::Sell, 1, 100));
        bh.push(Order::new( 2, OrderType::Sell, 2, 50));
        let mut o_buy_1 = Order::new( 3, OrderType::Buy, 10, 50);
        let t_2 = do_trade(o_buy_1.borrow_mut(),bh.pop().unwrap().borrow_mut());
        assert_eq!(t_2.price, 1);
        bh.clear();
        bh.push(Order::new( 1, OrderType::Sell, 2, 100));
        bh.push(Order::new( 2, OrderType::Sell, 1, 50));
        let mut o_buy_2 = Order::new( 4, OrderType::Buy, 10, 50);
        let t_2 = do_trade(o_buy_2.borrow_mut(),bh.pop().unwrap().borrow_mut());
        assert_eq!(t_2.price, 1);
    }
    #[test]
    fn construct_trade_retain_quantity() 
    {
        let mut o_sell = Order::new( 1, OrderType::Sell, 1, 100);
        let mut o_buy = Order::new( 2, OrderType::Buy, 1, 50);
        do_trade(o_buy.borrow_mut(), o_sell.borrow_mut());
        assert_eq!(50, o_sell.quantity);
        assert_eq!(0, o_buy.quantity);

        let mut o_sell = Order::new( 1, OrderType::Sell, 1, 50);
        let mut o_buy = Order::new( 2, OrderType::Buy, 1, 100);
        do_trade(o_buy.borrow_mut(), o_sell.borrow_mut());
        assert_eq!(50, o_buy.quantity);
        assert_eq!(0, o_sell.quantity);
    }

    
    // ------ Generic TESTS ------
    #[test]
    fn lowest_test()
    {
        assert_eq!(5, lowest(100, 5));
        assert_eq!(0, lowest(0, 5));
    }
}