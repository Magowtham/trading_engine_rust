use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
struct Price {
    integral:u64,
    fractional:u64,
    scalar:u64
}

impl Price {
    fn new(price:f64) -> Self {
        let scalar=100000;
        let integral=price as u64;
        let fractional=((price % 1.0) * scalar as f64) as u64;

        Price {
            scalar,
            integral,
            fractional
        }
    }
}

#[derive(Debug)]
struct Limit {
    price:Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price:f64) -> Self {
        Limit {
            price:Price::new(price),
            orders:Vec::new()
        }
    }

    fn add_order(self:&mut Self,order:Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size:f64,
    bid_or_ask:BidOrAsk
}

impl Order {
    fn new(bid_or_ask:BidOrAsk,size:f64) -> Order {
        Order {
            size,
            bid_or_ask
        }
    }
}

#[derive(Debug,PartialEq,Eq,Hash)]
struct OrderBook {
    asks:HashMap<Price,Limit>,
    bids:HashMap<Price,Limit>
}

impl OrderBook {
    fn new()-> Self {
        OrderBook {
            asks:HashMap::new(),
            bids:HashMap::new()
        }   
    }

    fn add_order(self:&mut Self,price:f64,order:Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price=Price::new(price);
                let limit=self.bids.get()
            },
            BidOrAsk::Ask => {

            }
        }
    }
}

fn main(){
  


// test.insert(k, v)


let query=format!("insert ({} , {}) in name row","sruajn",10);

println!("{}",query);



}