# Contenious_integration.yml
- タイミング:プルリク作成時、push時に実行される
- 通知:失敗した時だけDiscordに通知する
> 成功した場合に通知しないのは、Discordへの雑音を減らすため
- testに加え、Formatter, Lint, Build(+Coverage)も実行する
>Formatterはpre_commitに設定されているので不要かも
>Coverageのためにgithubでログインを試みたがタイムアウトになる...とりあえずコメントアウトしてます

#  dependency_check.yml
- タイミング:週一
> testやbuildに比べ、失敗する頻度が低いうえ、もし失敗してもすぐに対応できるとは限らないためこの頻度にしている。(毎日でもいいかも)
- 通知:失敗した時だけDiscordに通知する
- cargo crevによる分散レビューも検討中
>とりあえず最低限のcargo auditのみにしてます。

# 設定
1. 失敗時に通知するため、DiscordにGitHub Action専用のチャンネルを用意し、Webhook URLを作成する。
2. レポジトリのSettings > Secrets and variables > Actions に移動し、New repository secretボタンを押す
3. DISCORD_WEBHOOKを用意して、先ほどのWebhook URLをコピペする。AVATAR_URL(任意)も同様。
>下の例では。AVATAR_URL：<https://github.githubassets.com/images/modules/logos_page/Octocat.png>を使ってます。

<img width="546" alt="audit_fail_notify" src="https://user-images.githubusercontent.com/47593288/224492431-54eb434a-d28b-4b9c-b071-3e5e651b80d1.png">

# TODO/Nice to have
- Dependency Checkで検出された脆弱性を放置せざるをえない場合があるが、その解決策が示されるまでは手動で確認する必要があるので、この手間も削減できればいいな
- 誰が失敗したのか特定し、メンションつけて通知できれば改修までの時間を節約できる。(Triggered byの部分)
- cacheを適応して高速化する
- coverageの追加
- cargo crevの追加
