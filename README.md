# Synchrotron

Synchrotron is a full stack Rust web application written using [SQLx](https://github.com/launchbadge/sqlx) [Tonic](https://github.com/hyperium/tonic), [Juniper](https://github.com/graphql-rust/juniper) and [Iced](https://github.com/hecrj/iced).

It's used to map audio to text and currently supports a few foreign language courses. Check it out live at https://synchrotron.nsenger.com

## User manual

You will need to create an account to contribute to the mapping effort. Once you have an account, navigate to the document of interest and then you can link the audio to specific points in the text by using the following hotkeys and clicking on the document:

- SHIFT + P: play/pause the audio
- SHIFT + A: clicking on the document will create a green "anchor" in the document which will play the audio from its current position when clicked. This anchor will only be visible by you until it is upgraded to a blue anchor
- SHIFT + R: clicking on an anchor will remove that anchor from the document
- SHIFT + M: dragging an anchor will move it to a new position
- SHIFT + U: clicking on an anchor will upgrade it from green to blue (only administrator/moderator users can do this)

Currently the green unapproved anchors are only visible by their owners and moderators/administrators. The blue approved anchors are visible to all users.
