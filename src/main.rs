
mod input;
mod get_assert;
use get_assert::get_bbp_assert::total_count;
use get_assert::get_private_assert;

use input::Opts;

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::read();

    Opts::validate(&opts);
    
    if opts.bbp==true {
        let _resp = total_count(opts.cookie, opts.csrf_token).await;
    }else {
        let _resp =get_private_assert::total_count(opts.cookie, opts.csrf_token).await;
    }


   
}
