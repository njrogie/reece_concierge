# Recruiting and Hiring

Hi there. My name is Nick, and my specialization is usually embedded apps and driver development. This project started as a dumb idea and turned into an exercise that developed my skills in two areas: **Rust** and **Backend Design/Implementation**. I hope to put some of the noteworthy aspects of this project on display for those who are interested in hiring me.

## Project Background

One of my friends, Reece, has a Discord server where everyone enjoys playing games together, making jokes, and having a good time. We wanted to make a bot that would generate content that we could collectively laugh at.

### Purpose

The bot has one main routine:
- Obtain a daily trending article
- Pick a random message from each active user's entire message log in the server
- Simulate responses to the current article from each active user using that message.

Example:
```
Article: https://www.theonion.com/trader-joe-s-adds-new-fitting-rooms-where-customers-can-1850863022
"Trader Joe’s Adds New Fitting Rooms Where Customers Can See How Food Will Look In Their Mouth"

User1: Well, lord knows I wouldn't. (messaged 3 months ago)
User2: I'm gonna go grab pizza. (messaged 25 days ago)
User3: Not very dapper today, are you? (messaged 6 months ago)
```

If we're on the same page, you can see that this has the potential to create some hilarious article-quote pairs.

## Challenges

### Saving User Messages

This was the first challenge I wanted to tackle, as it was the least dependent on other factors (like an API for Current Events). You can see at [datastorage.rs](https://github.com/njrogie/reece_concierge/blob/aab549d9343dcaebe2531a6c0d2ea4ad06133705/src/datastorage.rs#L57) how this parsing begins in `start_gather_data(...)`. Search constraints are obtained via cli-like arguments given in the message, and the program then initiates the message aggregation in each non-excluded channel on the server.

Because development is still in its early stages, each user has their message stored in a .json file. I designed the `datastorage` module's interfaces such that it would be simple to refactor the software into a database-oriented storage schema later on down the line. I hope to eventually publicly deploy this bot and be able to gain experience in contanierized scalability as well as backend development.

#### Design Decisions

- Upon loading the app, if no stored messages are detected then the app will begin to process the server's messages. Messages are organized currently in a directory structure: `\[AppData folder\]/channel_id/month/user_id.json`, but as mentioned earlier will be sent to a database eventually.
- Each user thus has several files attributed to them. They will possibly have many active months in the server, and will have a representative number of files to sort through when choosing a message.
  - To perform an extremely fast pseudo-random calculation, the list of files is accumulated along with the filesizes of each. While not a 1\:1 value, filesize is a good enough indication of "weight" that a file should have in a pseudorandom choice.
  - After choosing the file, a random message is selected from the file.
- This process is repeated once for each active user. An active user has 2 files representing their messages over of the past two months.

### Current Events
TODO
