Autobrowser to browse steem and click on random likes. 
The main purpose was to learn how to command the mouse movements and keyboard interactions with Rust.
The code is quite specific with the attached png files made from my own screenshots,
but it is not intended to be used by many people. 

It should serve as a simple example for the actions mentioned above.

What it does:

 - When you start it up, it looks for pictures in hardcoded PATH and loads them in memory for usage.
 - Makes a screenshot, looks for firefox icon on the screenshot, gets the coordinates from the icon on the screen, moves the mouse to the given coordinates and clicks.
 - Opens a new tab, types in steem.com in the addressbar and clicks go.
 - Waits a little for steem.com to load, clicks on login, I have a password manager filling my details in automatically, hence there is no credential part just click on login
 - Clicks on New posts, scrolls down a little and starts liking random posts by taking screenshot an looking for the not yet clicked upvote buttons and clicks them.
 
Every scroll has one screenshot, one upvote, than scroll again.
A little random scrolling also built in for every 4th like.

