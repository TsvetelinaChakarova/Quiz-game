# Quiz-game
Programming with Rust project

Goal: Answer correctly as many questions as you can for the given time.
Rules: Questions with 4 possible answers appear to the user in random order. There is only one correct answer for each question. If the player clicks on the correct answer the screen turns green, he gets a point and the time increases a little. If the player clicks on the incorrect answer the screen turns red, he does not get a point and the time decreases a little. There are three lifelines (bonuses):
- bonus time: the player gets a bonus time
- skip question: the player skipps a question. No points are given or taken for that question 
- freeze time: the timer freezes until the player answers the current question.
Each lifeline can be used only once. While freeze time is active, the player cannot use the other lifelines even if they are still available. 
The game's over when the time's over or when the player answers all questions in the game.
After the time's over, the player can see how many questions he got correctly. There is a different message depending on the ratio of the correct and incorrect answers.
