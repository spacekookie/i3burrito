# i3burrito

Wrapping around i3lock like the fancy and delicious treat that it is! 🌯

## ???

I really like i3lock or rather i3lock-fancy which is a fancy
bash wrapper around the former to create a blurred background
and some text and stuff. I wanted to hook `fortune` into the text
without having to re-write the whole bash script (which uses some
truly bizzare features).

I tried to change the script but bash is...bash. So instead I'm
generating a new script with a substituted command string from
Rust (because it's not awful).

It'll put a script at `/tmp/locker.sh` and then run it.
