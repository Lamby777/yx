# Sort your files with `yx`!

Tired of having folders on folders on folders?
Wish there was a way to search for your stuff
without such a limiting tree structure?
Need a better way to organize your documents?
Images? Videos? Music? Who cares?! **Sort 'em all!**

With the help of `yx`, you can tag your files with your own... tags(?) and find them more easily! This lightweight program holds its entire state in a `.yx_index` file, which you can put in any folder or parent folder where you would like to use the program. You can make multiple `.yx_index` files, for separate libraries, or put one in a higher level directory to use `yx` anywhere. When you want to access your files, you can search for them using `yx list by` or apply more complicated searches using `yx constraint`. However, if you'd like to use your OS's file search (pretty overlooked feature, check out the link at the bottom of this file), you can also use `yx render named` to create a folder of hard links renamed to the original files. This way, you can name your files whatever you want, and still be able to search for them via tags in the search bar!

## Getting Started

| THIS APP IS A WORK-IN-PROGRESS.                             |
| ----------------------------------------------------------- |

Some of the features mentioned here aren't implemented yet.

<sup><sub>(...but they will be soon!)</sub></sup>

---

This tutorial assumes you're using Windows, but it should work
fine on Linux or macOS if you replace the Windows-specific
things, like backslashes in paths being replaced with slashes.

First, `cd` into the folder you would like to host your index
file in. For this example, we'll be trying out the program in
a folder called `stuff-to-tag` in the Documents folder. Then,
run the init command:
```
yx create
```

If you would rather not, for some reason, you could always
just specify an absolute or relative path in the command
itself, like this:
```
yx create C:\Users\sheep\Documents\stuff-to-tag
```

Now, you'll need to add in some of your own tags.
```
yx add fireflies.txt music song-lyrics
yx add amogus-meme.png meme sus
yx add amogus-drip.mp3 music meme sus
```

Clearly, these 3 files have at least some characteristics in common! However, normally, you wouldn't be able to group these 3 files properly. You could try putting the 2 memes together, but then the text file would have to go somewhere else. You could put the 2 music-related files together, but then the image would need a different location as well.

You can decide any file structure that seems right to you, but there will always be a different way to group the files that you might want to see. For those cases, you are now able to search for such desired files using the `yx` command.

Let's say you have the files sorted like this:
```
stuff-to-tag (folder)
| - memes (folder)
|   | - amogus-meme.png
|   | - amogus-drip.mp3
|
| - .yx_index
| - fireflies.txt
```

You want to find all music-related files, but you'd have to go searching through all your stuff! Luckily, we already have a couple tags set up.

From the folder `.yx_index` is in (or any subfolder from there), run this command to add a command constraint:
```
yx constraint tag music
```

There are lots of constraints to use for more fine-grained control.

Cool! Now any `yx` commands we run using this index file will only apply to files tagged as `music`. Let's check out our files.
```
yx render
```

Without any options, the `render` subcommand creates a folder called `yx-out` in the same folder, and creates some hard-links to every matching file in the index. However, since we put a `constraint` before running the command, it only created links to our files with the tag `music`. Nice!

Now, you probably don't want to limit yourself to these 2 files forever... Let's `free` the program from our previous `constraint`s.
```
yx free
```

And now, we're back to where we started! Now, go check out your files!

## More Info / Useful Links
- [Hard Link vs Copied File](https://unix.stackexchange.com/a/65003/506584)
- [Track This Repo's Progress](https://github.com/Lamby777/yx/issues/1)
- [Windows Search Syntax Tips](https://windowsloop.com/useful-file-explorer-search-syntax-commands/)
