# morsel
Command line tool to translate Morse code input to text in real time.


Download for mac (Homebrew) with:

```
brew tap samlee514/tap
brew install morsel
```

__Usage:__

Use --manual (-m) to enter manual mode and --dit-length=... (-d=...) to set the length of a dit in milliseconds (default 500)

** Note that dit length is not used in manual mode and should be greater than your machine's key repeat delay.

** If you find it to be too slow, you can decrease your machine's key repeat delay or double tap instead of holding.

Regular mode:
- Tap any key to input a dit (.). Hold any key (or double tap within one dit length) to input a dah (_).
- Do not input anything for one dah length to translate current input.
- Do not input anything for 7 dit lengths to write a space.

Manual Mode:
- Type '.' and '_' manually. Press space once to translate and another time to input spaces.

