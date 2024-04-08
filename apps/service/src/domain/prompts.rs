pub struct Prompts {
    pub summary_pre_prompt: String,
}

impl Prompts {
    pub fn new(context: String) -> Self {
        let summary_pre_prompt = format!(
            "
            ##要件定義
            以下は技術ブログサイトに投稿されたmarkdown形式の記事です。この記事を以下制約に従って要約してください。
            
            ##制約
            - 1000文字以下で要約すること。
            - 要約の文章はmarkdown形式で記述すること。
            - 最初にどのような内容が書かれているかがわかるようにすること。
            - 特に重要なコードは要約に含めること。
            - 画像は要約に含めないこと。

            ##入力
            {}
            "
            ,context
        );
        Prompts { summary_pre_prompt }
    }
}
