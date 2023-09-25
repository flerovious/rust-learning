# Usage
### `cargo run -- print --path data/lyrics.txt`
Output:
```
Det har regnat i dagar
Det regnar och det slutar aldrig
Det regnar genom kragen
Står lutad mot en stenstaty

Ser sovande svanar, ser våta fasader
Och ödsliga parker där vi gick runt

Om du visste vad det var, varför sa du inget?
Jag tror du visste vad det var, men du sa ju inget
Du bara låtsas som det regnar
Du låtsas som det regnar

Jag ägnade dagar åt det som är ingenting nu
Jag bad dig och förklara
Jag ville ju precis som du
Se leriga åkrar, se tomma perronger
Och ödsliga parker där vi gick runt

Om du visste vad det var, varför sa du inget?
Jag tror du visste vad det var, men du sa ju inget
Du bara låtsas som det regnar
Du låtsas som det regnar

Du bara låtsas som det regnar
Du låtsas som det regnar
```

### `cargo run -- translate --path data/lyrics.txt`
Output:
```
Downloading https://huggingface.co/Helsinki-NLP/opus-mt-sv-en/resolve/main/vocab.json [1.23MiB]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/Helsinki-NLP/opus-mt-sv-en/resolve/main/source.spm [796.19KiB]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/Helsinki-NLP/opus-mt-sv-en/resolve/main/config.json [1.35KiB]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/Helsinki-NLP/opus-mt-sv-en/resolve/main/rust_model.ot [501.34MiB]........ ✓ Done! Finished in 14 seconds
 It's been raining for days
 It's raining and it never stops
 It's raining through the collar
 Stands leaning against a stone statue
 I'm sorry. I'm sorry.
 See sleeping swans, see wet facades
 And deserted parks where we walked around
 I'm sorry. I'm sorry.
 If you knew what it was, why didn't you tell me?
 I think you knew what it was, but you didn't tell me.
 You just pretend like it's raining
 You pretend like it's raining.
 I'm sorry. I'm sorry.
 I spent days on what's nothing now
 I asked you to explain.
 I wanted just like you.
 See muddy fields, see empty platforms
 And deserted parks where we walked around
 I'm sorry. I'm sorry.
 If you knew what it was, why didn't you tell me?
 I think you knew what it was, but you didn't tell me.
 You just pretend like it's raining
 You pretend like it's raining.
 I'm sorry. I'm sorry.
 You just pretend like it's raining
 You pretend like it's raining.
```