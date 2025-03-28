# LeadSheet ML
LeadSheetML is a markup language and compiler for lead sheets; combining chords, 
lyrics, and metadata into a clean, structured format that can be rendered as Markdown, HTML, 
or other output formats. It's designed for musicians, songwriters, educators, and developers 
who want a text-based, version-controllable format for music charts.

### Features

- Parse lead sheets written in human-readable syntax 
- Convert to Markdown or HTML with proper alignment of chords and lyrics 
- Support for directives (title, artist, key, etc.)
- Measure and inline notation for flexibility in formatting 
- Robust parser built with pest 
- Full test coverage with clean, modular rendering engine
- Built-in support for extensions and slash chords (e.g. [Cmaj7b9/E])

### LeadSheet ML Syntax

#### Directives:
Start your files with optional metadata using `@`
```
@title: Twinkle Twinkle Little Star
@artist: Traditional
@key: C Major
@time: 4/4
```

#### Sections & Blocks
Each section starts with a `#`:
```
#Verse
[C] Twinkle, twinkle, little [G] star  
How I wonder what you [C] are!  
[C] Up above the world so [G] high  
Like a diamond in the [C] sky.
```

You can also write chord-only blocks like this:
```
#Solo
| [C] [G] || [C] [G] |
```

### Usage

```
leadsheetml <path-to-song>
```
e.g.
```
leadsheet SongBook/examples/TwinkleTwinkle.impl #Generates Markdown
leadsheet SongBook/examples/TwinkleTwinkleAdvanced.impl --html #Generates HTML

```

By default it will generate a markdown file, but it can also generate an html file with the `--format html` flag.
`--format markdown` also works.

##### Transposing
To transpore a song simply add the argument `--transpose` followed by the number of steps shift the song by. To transpose down, pass `transpose_down`.

### Philosophy
LeadSheetML is designed to be extensible and hackable you can build renderers, analyzers, transposers,
or even melody engines on top of it. It's part of a larger vision (code-named MuTeX) to create high-quality,
text-native musical tooling.

### Roadmap
- Markdown & HTML (Completed)
- Full parser & AST structure (Completed)
- Modular rendering engine via traits (Completed)
- Transposition Engine (Completed)
- Voice/Melody Encoding (Up next)
- Syntax Highlighting/VSCode/vim/Intellij Plugins
- PDF rendering
- Web Editor

## Development



To use or develop LeadSheetML, clone this repo and markup_engine:

```
git clone https://github.com/omnomchomsky/leadsheetml
git clone https://github.com/omnomchomsky/markup_engine
```

Then in your Cargo.toml, make sure the dependency points to the local path:

```
[dependencies]
markup_engine = { path = "../markup_engine" }
```
