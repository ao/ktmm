# ktmm (Keep That Mouse Moving!)


## What is this?
Ever needed to stop your computer from going into Idle mode?

Now it's super simple!

## How does it work?
This little guy will move your mouse back and forth a tiny little bit every second; and you won't even notice it!

## When would I ever use this?
Works well in companies where you can't stop your computer from Idling and things start showing as you're away when you've decided to go relax on the beach!

## Why was this utility made?
The author could not stop the screensaver from being triggered after 5 minutes (permissions issues) and while waiting for gcc/make/other to complete compiling the project files, the machine would lock up and turn off (thanks big brother company..).

This utility makes sure that the screensaver is never triggered and the compile job completes with smiles all around!

It also makes sure that the computer never, ever, ever goes into Idle mode..

## 1. Get Setup

```
git clone https://github.com/ao/ktmm.git
cd ktmm
pip install -r requirements.txt
```

## 2. Now run it
`python ktmm.py`

There's no need to stop it, unless you really do want to go idle!


## If you want to run it in a virtualenv then do this instead:

```
git clone https://github.com/ao/ktmm.git
cd ktmm
virtualenv -p python3 env
. env/bin/activate
pip install -r requirements.txt
python ktmm.py
```

# Looking for a cross-platform version?
If you need a cross-platform version, you can use the [Java version](https://github.com/ao/ktmm-java)
