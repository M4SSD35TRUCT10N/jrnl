![Rust](https://github.com/M4SSD35TRUCT10N/cerebro/workflows/Rust/badge.svg)

#  jrnl
jrnl is a cli tool for writing a journal.
It is not intended to be another zettelkasten application or a note taking app,
although it uses the power commonmark markdown and thus could be used as a
zettelkasten.

It strives to have no dependencies, just pure standard stable rust and to be a
single executable without clutter.

This is why the manual and the default config are hardcoded into the executable.

> [!CAUTION]
> This is alpha state software within its implementation phase.
> Bugs are imminent and features will not work or create unreadable files.
> Sometimes things are simply not implemented. I strive to notify you about that
> when you use that feature or configuration entry.

## Goals
Be a viable open source rust written digital journal writing app. Using plain
text files.

## Non-Goals
Replace all valuable zettelkasten or note taking apps.

## How to use it
Basically like this:
```
jrnl today "It was a productive day without any interruptions."
```

As you can see in its simplest form jrnl will generate a commonmark markdown
file with a default header. Your can change this with a template of your own.

### Arguments
jrnl allows you to write a journal entry with different arguments ahead.

***init***

Creates the default config file jrnl.cfg in the same directory as the
program itself is located at unless you provide the following:

```    
cfg path-to-config/my.cfg
```

> [!IMPORTANT]
> Has to be the first argument.
> Does not work with journals, editor and template configuration entries.

***set***

Lets you change the value-key pair in the config file. Will change the
default config unless provided with cfg path-to-config/my.cfg.
Changing the encryption in my.cfg located at ~/jrnl would look like
this: jrnl set enrcyption=enabled cfg ~/jrnl/my.cfg.
It's possible to change multiple config entries separating them with a
comma ',' (e.g. encryption=enabled,mode=folders).

> [!IMPORTANT]
> Has to be the first argument.

***cfg path-to-config/my.cfg***

Loads your config file. If omitted jrnl defaults to jrnl.cfg in the
directory where jrnl executable lies. If jrnl.cfg does not exist or can
not be read it will stop execution and prompt you.
it will be created for you at the first start with standard values.

***yesterday***

Will use the date and time of the day before for the creation of the
journal entry.

***today***

Will use the current date and time for the creation of the journal
entry. The argument 'today' can be omitted. jrnl will assume you write
your entry for today.

***tomorrow***

Will use the date and time of the day after for the creation of the
journal entry.

***specific journal name***

Will write to the corresponding journal. ```jrnl work today [...]```

***add***

Using 'add' will alow you to add to an already written entry.
For example ```jrnl add ~/jrnl/2021/05/05.md``` or
```jrnl add ~/jrnl/2021-05-05.md``` will let you add some text to
your entry and updating the time of change in the header of the opened
file. With ```jrnl add today``` (today can be omitted) you could add to the
current entry. ```jrnl add tomorrow``` and ```jrnl add yesterday``` will work also.
If you omit the file the current entry will be modified and the
provided String will be added as a new line.
For example ```~/jrnl/2021/05/05.md``` when you add something and today would
be the 5th May of 2023.

### Configuration
The configuration file is a simple key-value store. You can comment with a
'#' as first character per line. Currently there is not much to set.

***journals***

Here you define the journals (path-to-and-name-of-the-journal) in a
comma ',' separated list. E.g. journals=default,
/home/pale-rider/jrnl/work,/home/pale-rider/jrnl/private.
The last entry of the above given path is the journal name which can be
an cli argument as mentioned above. Default writes to the directory the
jrnl executable lies in. The default is 'default'.
You should change this.

***mode***

Can be either ```mode=folders``` or ```mode=files```.
The default is ```mode=files```.

Mode 'folders' will generate a journal entry like this:
```~/jrnl/2021/05/05.md```.

Mode 'files' will generate a journal entry like this:
```~/jrnl/2021-05-05.md```.

***encryption***

This enables encryption with ```encryption=enabled``` for your notes.
The default is ```encryption=disabled```.
Maybe you should change this, if you plan to store them on someone
else' computer.

***editor***

This gives you the possibility to use an external editor of your choice
for finishing your journal entry. For example ```editor=nvim``` will start
NeoVIM with the freshly generated file. The default is ```editor=none```.

***template***

You have the option to set up a template commonmark markdown file for
your journal entries with e.g. ```template=~/jrnl/my-jrnl-entry.md```.
By default it is set to ```template=none```.

***stardate***

When enabled with ```stardate=enabled``` it will generate a stardate to
display in the header of the journal entry.
Default is ```stardate=disabled```.

***editing_mark***

With ```editing_mark=enabled``` every editing through ```jrnl add``` is traceable
because it will add the timestamp above the added lines.
Default is ```editing_mark=disabled```.

> [!TIP]
> The training you should finish first. Shortcuts the path to the dark side they are.
