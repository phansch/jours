[![Build Status](https://travis-ci.com/phansch/jours.svg?branch=master)](https://travis-ci.com/phansch/jours)

# Jours

Essentially a glorified version of:

```shell
echo "* $(date +%X): I just did this and that" >> my_journal.txt
tail my_journal.txt
```

## Why?

* I want to keep a journal but found myself doing it only if there are absolutely no barriers to adding to the journal.
* I can more easily integrate it with other things, such as automatically logging when the screen was locked and unlocked, together with a timestamp.
