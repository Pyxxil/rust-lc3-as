 - Implement more notifier types. [ ]
  - This might be superseded by the idea of callbacks as opposed to notifiers
    themselves, as callbacks would be more versatile, i.e. they allow the
    program to more accurately deal with the information, e.g. pass it to their
    own function to turn it into JSON output, or something else. [ ]
 - Quality of life:
  - BLKW shouldn't be allowed to consume the LABEL if it's on a different line [ ]
 - Warnings:
  - LSHIFT shouldn't allow a shift by 0 (warning or error?) [ ]
  - JSRR should warn about jumping to R7 [  ]
  - BR/JSR etc should warn with offsets of #-1 [  ]
 - Errors:
  - Should error on labels not being found/being outside of range for:
    - BR [ ]
    - JSR [ ]
    - BLKW (not being found only) [ ]
    - FILL (not being found only) [ ]
    - ST [ ]
    - LD [ ]
    - STI [ ]
    - LDI [ ]
    - LEA [ ]
    - SET (not being found only) [ ]
  - .SUB should attempt to optimise (at the moment there is a different output
  between .SUB R3, R0, R3 (simply inverts R3, adds r0 to it, then puts it in R3) and .SUB R3, R3, R0/.SUB R3, R0 (which inverts R0, adds it to R3, stores in R3, then inverts R0)) [  ]
