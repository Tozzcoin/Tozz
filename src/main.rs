use warp::{Filter, Reply};

#[derive(Clone)]
struct TozzBlockchain;

impl TozzBlockchain {
    // لو مش هتستخدم المتغيرات amount و contract_address، هضيف _ عشان أزيل التحذيرات
    fn send_tokens_to_contract(&self, _amount: u64, _contract_address: &str) -> Result<(), String> {
        // تنفيذ الإرسال
        Ok(())
    }

    fn send_tokens_from_contract(&self, _amount: u64, _contract_address: &str) -> Result<(), String> {
        // تنفيذ الإرسال
        Ok(())
    }
}

// دالة التعامل مع إرسال التوكنات إلى العقد
async fn handle_send_tokens_to_contract(body: SendTokensRequest, blockchain: Arc<Mutex<TozzBlockchain>>) -> Result<impl Reply, warp::Rejection> {
    let blockchain = blockchain.lock().unwrap();
    match blockchain.send_tokens_to_contract(body.amount, &body.contract_address) {
        Ok(_) => Ok(warp::reply::json(&TransactionResult {
            success: true,
            message: "Tokens sent successfully".to_string(),
        })),
        Err(e) => Ok(warp::reply::json(&TransactionResult {
            success: false,
            message: e,
        })),
    }
}

// دالة التعامل مع إرسال التوكنات من العقد
async fn handle_send_tokens_from_contract(body: SendTokensRequest, blockchain: Arc<Mutex<TozzBlockchain>>) -> Result<impl Reply, warp::Rejection> {
    let blockchain = blockchain.lock().unwrap();
    match blockchain.send_tokens_from_contract(body.amount, &body.contract_address) {
        Ok(_) => Ok(warp::reply::json(&TransactionResult {
            success: true,
            message: "Tokens sent successfully".to_string(),
        })),
        Err(e) => Ok(warp::reply::json(&TransactionResult {
            success: false,
            message: e,
        })),
    }
}

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(TozzBlockchain));

    // Routes
    let send_tokens_to_contract = warp::path("send_tokens_to_contract")
        .and(warp::body::json())
        .and(with_blockchain(blockchain.clone()))
        .and_then(handle_send_tokens_to_contract);

    let send_tokens_from_contract = warp::path("send_tokens_from_contract")
        .and(warp::body::json())
        .and(with_blockchain(blockchain.clone()))
        .and_then(handle_send_tokens_from_contract);

    let routes = send_tokens_to_contract.or(send_tokens_from_contract);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Helper function to attach blockchain to the filter
fn with_blockchain(blockchain: Arc<Mutex<TozzBlockchain>>) -> impl Filter<Extract = (Arc<Mutex<TozzBlockchain>>,), Error = Infallible> + Clone {
    warp::any().map(move || blockchain.clone())
}
