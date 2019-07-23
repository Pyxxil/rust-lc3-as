;
; Initialization
;
		.ORIG	x3000
		LD		R6, EMPTY		; R6 is the stack pointer
		LD		R5, PTR			; R5 is pointer to characters
		AND		R0,	R0,	#0
		ADD		R0,	R0,	#10		; Print a new line
		OUT
;	
REDO	LDR		R3, R5, #0		; R3 gets character
;
; Test character for end of file
;		
		ADD		R4, R3, #-10	; Test for end of line (ASCII xA)
		BRz		EXIT			; If done, quit
		LD		R4, ZERO
		ADD		R3, R3, R4		; Get the decimal value from ASCII to R3
		JSR		CONV
		ADD		R5, R5, #1
		LD		R0, FACTOR		; '!'
		OUT
		LD		R0, EQUAL		; '='
		OUT
; Start calculation (The operand is at R3)
		ADD 	R4, R3, #0
		ADD 	R1, R3, #-1

; Loop	
OLoop	AND		R2, R2, #0		
ILoop	ADD		R2, R2, R4
		ADD		R1, R1, #-1	
		BRp		ILoop

		ADD 	R4, R2, #0
		ADD		R3, R3, #-1
		ADD 	R1, R3, #-1
		BRp		OLoop
		ADD		R3, R4, #0
;
		JSR		CONV
		AND		R0,	R0,	#0
		ADD		R0,	R0,	#10		; Print a new line
		OUT
		BRnzp	REDO		
;
; A subroutine to output a 3-digit decimal result.
;
CONV	ADD		R1, R7, #0		; R3, R4, R5 and R7 are used in this subroutine
		JSR		Push
		ADD		R1, R3, #0		; R3 is the input value
		JSR		Push
		ADD		R1, R4, #0
		JSR		Push
		ADD		R1, R5, #0
		JSR		Push
		AND 	R5, R5, #0
OUT100	LD		R4, HUNDRED
		ADD		R4, R3, R4		; R3 - #100
		BRn		PRI100
		LD		R4, HUNDRED
		ADD		R3, R3, R4		; R3 - #100
		ADD		R5, R5, #1
		BRnzp	OUT100
PRI100	LD		R0, ASCII		; Load the ASCII template
		ADD		R0, R0, R5		; Convert binary count to ASCII
		OUT						; ASCII code in R0 is displayed.
		AND 	R5, R5, #0
OUT10	ADD		R4, R3, #-10
		BRn		PRI10
		ADD		R3, R3, #-10
		ADD		R5, R5, #1
		BRnzp	OUT10
PRI10	LD		R0, ASCII		; Load the ASCII template
		ADD		R0, R0, R5		; Convert binary count to ASCII
		OUT						; ASCII code in R0 is displayed.		
		LD		R0, ASCII
		ADD		R0, R0, R3		; Convert binary count to ASCII
		OUT						; ASCII code in R0 is displayed.
		JSR		Pop
		ADD		R5, R1, #0
		JSR		Pop
		ADD		R4, R1, #0
		JSR		Pop
		ADD		R3, R1, #0
		JSR		Pop
		ADD		R7, R1, #0
		RET
EXIT	HALT					; Halt machine
Push	STR 	R1, R6, #0		; Stack Push
		ADD 	R6, R6, #-1 
		RET 
Pop 	ADD 	R6, R6, #1		; Stack Pop
		LDR 	R1, R6, #0
		RET
; End of the subroutine

PTR		.FILL	x3500
EMPTY 	.FILL 	x4000 
ASCII	.FILL	x0030			; '0'
ZERO	.FILL	xFFD0			; -'0'
HUNDRED	.FILL	xFF9C			; -#100
EQUAL	.FILL	x003D			; '='
PLUS	.FILL	x002B			; '+'
MINUS	.FILL	x002D			; '-'
FACTOR	.FILL	x0021			; '!'
MULT	.FILL	x002A			; '*'
VAL		.BLKW	1
		.END


