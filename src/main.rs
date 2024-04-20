use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .nest_service("/", ServeFile::new("index.html"))
        .route("/api/answer", post(serve_ans));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn check_answer(qnum: usize, answer: &str) -> Result<(), ()> {
    let answers = [
        "8公克:8g",
        "海豹",
        "保麗龍",
        "紅蘿蔔",
        "嘴巴:牙齒",
        "瀑布",
        "絕代佳人",
        "一鳴驚人",
        "鎖匠",
        "瀑布:剪刀石頭布",
        "偷什麼東西不犯法？:偷笑:笑",
        "人",
        "拔河",
        "細菌的兒子:細菌兒子",
        "親戚關係:親戚",
        "謎底:f12:ctrl+shift+i",
        "每年的三月三十日:每年",
        "辭海:辭海",
        "在床上:床上:病床上",
        "前功盡棄",
        "鋼琴:鍵盤:網管小組",
        "球:球",              // 當貝克漢姆主罰點球時，他會擊中哪裡？
        "撲克牌:一副撲克牌",  //什麼東西有13個心臟，但沒有其他器官？
        "一、二、三:1、2、3", // 哪三個不為零的數字，無論相加還是相乘，都會給出相同的答案？
        "值:值",              // 半真半假。（打一字）
        "乖:乖",              // 乘人不在。（打一字）
        "合:合",              // 拿不出手。（打一字）
        "乍:乍",              // 昨日不可留。（打一字）
        "朋:朋",              // 六十天。（打一字）
        "暈:暉",              // 東洋兵。（打一字）
        "也:也",              // 我沒有他有，天沒有地有。（打一字）
    ];

    if answers[qnum]
        .split(":")
        .find(|correct| correct.to_lowercase() == answer.to_lowercase())
        .is_some()
    {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Deserialize)]
struct CheckAnswer {
    qnum: usize,
    answer: String,
}

// basic handler that responds with a static string
async fn serve_ans(Json(payload): Json<CheckAnswer>) -> StatusCode {
    if check_answer(payload.qnum, &payload.answer).is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}
