pub const INDEX: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Gender Affirming Care Search</title>
</head>
<body>
    <h1>Gender Affirming Care Search</h1>

    <form>
        <label for="provider_name">Provider Name:</label>
        <input type="text" id="provider_name" name="provider_name"><br><br>

        <label for="care_type">Care Type:</label>
        <input type="text" id="care_type" name="care_type"><br><br>

        <label for="subreddits">Subreddits:</label>
        <textarea id="subreddits" name="subreddits"></textarea>
    </form>
</body>
</html>
"#;
