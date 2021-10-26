use std::{cmp::Ordering, fmt::{self}};


// ----- ----- OrderType ENUM ----- -----
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum OrderType {
    Buy,
    Sell,
}
impl fmt::Display for OrderType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        fmt::Debug::fmt(self,f)
    }
}
impl OrderType 
{
    pub fn from_str(input: &str) -> Option<OrderType> {
        match input {
            "Buy"   => Some(OrderType::Buy),
            "Sell"  => Some(OrderType::Sell),
            _      => None,
        }
    }
}

// ----- ----- ----- ----- ----- ----- -----


// ----- ----- Trade STRUCT ----- -----
#[derive(Debug)]
pub struct Trade {
    pub buy_id: usize,
    pub sell_id: usize,
    pub price: u32, // this should be the sell price.
    pub quantity: u32
}
impl Trade {
    pub fn new(buy_id: usize, sell_id:usize, price:u32, quantity:u32) -> Trade
    {
        Trade { buy_id: buy_id, sell_id: sell_id, price: price, quantity: quantity }
    }
    pub fn to_string(&self) -> String{
        format!("Trade: {} BTC @ {} USD between {} and {}", 
            self.quantity, self.price, self.buy_id, self.sell_id)
    }
}
impl PartialEq<Trade> for Trade {
    fn eq(&self, other: &Trade) -> bool 
    {
        self.buy_id == other.buy_id && 
        self.sell_id == other.sell_id && 
        self.price == other.price && 
        self.quantity == other.quantity
    }
}

//again, could return string here but this gives us more wiggleroom
pub fn do_trade(buy:&mut Order, sell:&mut Order) -> Trade
{
    let quantity;
    if sell.quantity < buy.quantity
    {
        quantity = sell.quantity;
        buy.quantity -= sell.quantity;
        sell.quantity = 0;
    }
    else 
    {
        quantity = buy.quantity;
        sell.quantity -= buy.quantity;
        buy.quantity = 0;
    }
    
    Trade::new(
        buy.id, 
        sell.id,
        lowest(buy.price,sell.price),
        quantity
    )
}

// ----- ----- ----- ----- ----- ----- -----


// ----- ----- Order STRUCT ----- ----- 
#[derive(Debug, Eq)]
pub struct Order {
    pub id: usize,
    pub order_type: OrderType,
    pub price: u32,
    pub quantity: u32
}
impl Order {
    pub fn new(id: usize, order_type:OrderType, price:u32, quantity:u32) -> Order
    {
        Order { id: id, order_type: order_type, price: price, quantity: quantity }
    }
    pub fn empty() -> Order //used while writing the program
    {
        Order { id:0, order_type: OrderType::Buy, price: 0, quantity: 0 }
    }
    pub fn to_string(&self) -> String //used for testing
    {
        format!("ID: {}, Type: {}, Price: {}, Quantity: {}",self.id,self.order_type.to_string(),self.price,self.quantity)
    }
    pub fn equal_to(&self, other:&Order) -> bool //used for testing
    {
        self.id == other.id &&
        self.order_type == other.order_type &&
        self.price == other.price &&
        self.quantity == other.quantity
    }
}
impl PartialEq<Order> for Order { //this is for the binary heap collection
    fn eq(&self, other: &Order) -> bool 
    {
        self.price == other.price 
    }
}
impl PartialOrd for Order { //ditto
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Order { //ditto
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price).reverse()
    }
}
//input should be of form {{id}}: {{Buy|Sell}} {{quantity}} BTC @ {{price}}
pub fn construct_order(inp: &String) -> Option<Order>
{
    let order_split = inp.split_whitespace().collect::<Vec<_>>();
    if order_split.len() < 7 //wrong input form
    {
        return None
    }
    let pat: &[_] = &['.',':']; //both . and : in task so assume both
    let id = match order_split[0].trim_matches(pat).parse::<usize>()
    {
        Ok(x) => x,
        Err(e) => {
            println!("ERR: Couldn't parse {} to id", order_split[0]);
            println!("e: {}", e.to_string());
            return None;
        }
    };
    let order_type = match OrderType::from_str(order_split[1])
    {
        Some(x) => x,
        None => {
            println!("ERR: Couldn't parse {} to order type", order_split[1]);
            return None;
        }
    };
    let price = match order_split[5].parse::<u32>()
    {
        Ok(x) => x,
        Err(e) => {
            println!("ERR: Couldn't parse {} to price", order_split[5]);
            println!("e: {}", e.to_string());
            return None;
        }
    };
    let quantity = match order_split[2].parse::<u32>()
    {
        Ok(x) => x,
        Err(e) => {
            println!("ERR: Couldn't parse {} to quantity", order_split[2]);
            println!("e: {}", e.to_string());
            return None;
        }
    };

    Some( //all tests OK, proceed
        Order::new(
            id, 
            order_type, 
            price, 
            quantity
        )
    )       
}

//UNUSED
//Testing function for a multiline input
//Could be used for a multiline file input at some point
pub fn get_orders(inp: String) -> Vec<Order>
{
    let mut orders: Vec<Order> = Vec::new();

    let orders_str = inp.split("\n");
    for order in orders_str
    {
        match construct_order(&String::from(order))
        {
            Some(o) => orders.push(o),
            None => println!("WARNING: Couldn't make string '{}' into order", order)
        }
    }

    orders
}

// ----- ----- ----- ----- ----- ----- -----


// ----- ----- HELPER FUNCTIONS ----- -----
pub fn lowest(a:u32, b:u32) -> u32 
{
    if a < b
    {
        a
    }
    else 
    {
        b
    }
}

// ----- ----- ----- ----- ----- ----- -----