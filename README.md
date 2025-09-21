# Go Getta

A social media website coded as a hobby project. 

Users can register with a username and password and upload posts, follow other users and interact with posts by giving them a positive or negative review or by commenting them.
Posts can either be published to the 'public space' for everyone to see, or inside a closed space ('room'), that users have the ability to create. Rooms can be public, allowing everyone to join, or private, in which case only the room owner can add people to the room.
Posts can hold media as appendages, including images, audio and video. This content is stored on and served by the server using filesystem based routing.
The contents of the posts themselves, as well as all other data (like user data, associations, etc.) are stored inside a SQLite database.

The website is made using vanilla html, css and javascript and the ui is composed of custom icons.

Since this project was never meant to be open to the general public, the registration requires a single-use 'account creation key', which is to be handed to the user by the server administrator.
The project was deployed on a raspberry-pi and tested by my friends.
