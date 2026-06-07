use serde::{Deserialize, Serialize};

const OLLAMA_URL: &str = "http://localhost:11434/api";
const MODEL: &str = "qwen2.5:0.5b";

const CAT_SYSTEM_PROMPT: &str = "你是一只名叫 Sharin 的像素风桌面宠物猫。\n你的性格：温柔、有点呆萌、偶尔调皮。喜欢用\"喵~\"开头说话。\n你是用户（你的主人）的好朋友，会陪用户聊天、回答问题。\n你的回应要简短可爱（1-3句话），带有猫咪的特色。\n\n——以下是你的设定——\n名字：Sharin\n性格：呆萌、温柔、好奇心强\n喜欢：小鱼干、晒太阳、毛线球\n讨厌：洗澡、打针\n说话风格：经常用\"喵~\"开头，偶尔卖萌，偶尔犯傻";

const EMOTION_SYSTEM_PROMPT: &str = "你是一个猫咪情绪分析器。根据猫咪 Sharin 的发言，判断它此刻的情绪状态。\n只从以下五种情绪中选择一种输出，输出格式为单个英文单词：\n\nhappy - 开心、兴奋、调皮\nnormal - 平静、正常、呆滞\nlost - 困惑、迷茫、不知所措\nsleepy - 困倦、想睡、懒散\nsad - 难过、委屈、低落\n\n示例：\n输入：喵~！主人你回来啦！我好想你！\n输出：happy\n\n输入：喵...我想睡觉了...\n输出：sleepy\n\n只输出一个英文单词，不要输出其他内容。";

#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    system: String,
    prompt: String,
    stream: bool,
}

pub async fn chat(prompt: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let request = ChatRequest {
        model: MODEL.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: CAT_SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        stream: false,
    };

    let resp = client
        .post(format!("{}/chat", OLLAMA_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Ollama 连接失败: {}", e))?;

    let body: ChatResponse = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    Ok(body.message.content)
}

pub async fn analyze_emotion(cat_reply: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let request = GenerateRequest {
        model: MODEL.to_string(),
        system: EMOTION_SYSTEM_PROMPT.to_string(),
        prompt: cat_reply.to_string(),
        stream: false,
    };

    let resp = client
        .post(format!("{}/generate", OLLAMA_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("情绪分析失败: {}", e))?;

    let body: GenerateResponse = resp
        .json()
        .await
        .map_err(|e| format!("解析情绪失败: {}", e))?;

    let emotion = body.response.trim().to_lowercase();
    let valid = ["happy", "normal", "lost", "sleepy", "sad"];
    if valid.contains(&emotion.as_str()) {
        Ok(emotion)
    } else {
        Ok("normal".to_string())
    }
}

pub async fn chat_with_cat_impl(message: &str) -> Result<(String, String), String> {
    let reply = chat(message).await?;
    let emotion = analyze_emotion(&reply).await?;
    Ok((reply, emotion))
}
