 - Implement more notifier types. [ ]
  - This might be superseded by the idea of callbacks as opposed to notifiers
    themselves, as callbacks would be more versatile, i.e. they allow the
    program to more accurately deal with the information, e.g. pass it to their
    own function to turn it into JSON output, or something else. [ ]
 - Quality of life:
  - BLKW shouldn't be allowed to consume the LABEL if it's on the next line [ ]
 - Warnings:
  - LSHIFT shouldn't allow a shift by 0 (warning or error?) [ ]
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
