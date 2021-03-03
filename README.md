# Synchrotron

Synchrotron is a full stack Rust web application written using [SQLx](https://github.com/launchbadge/sqlx) [Tonic](https://github.com/hyperium/tonic), [Juniper](https://github.com/graphql-rust/juniper) and [Iced](https://github.com/hecrj/iced).

It's used to map audio to text and currently supports a few foreign language courses. Check it out live at https://synchrotron.nsenger.com

## User manual

You will need to create an account to contribute to the mapping effort. Once you have an account, you can link the audio to specific points in the text using the following hotkeys and then clicking on the document:

- SHIFT + P: play/pause the audio
- SHIFT + A: clicking on the document will create an "anchor" in the document which will play the audio from its current position when clicked
- SHIFT + R: clicking on an anchor will remove that anchor from the document
- SHIFT + M: dragging an anchor will move it to a new position
- SHIFT + U: clicking on an anchor will upgrade it from green to blue (only administrator/moderator users can do this)

Currently both approved and unapproved anchors are visible to all users.
