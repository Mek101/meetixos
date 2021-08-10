/*! x86_64 interrupt service routines */

.extern interrupt_handler
.extern syscall_handler

.macro DEFAULT_ISR num
    .global     isr_\num
    .type       isr_\num, @function
    isr_\num:
        push    $0       /* push no-error code value */
        push    $\num    /* push interrupt number */
        jmp     isr_common_entry
    .size       isr_\num, . - isr_\num
.endm

.macro ERROR_ISR num
    .global     isr_\num
    .type       isr_\num, @function
    isr_\num:
        /* in this case there is no-need to push the error code,
         * since this is an interrupt-gate, not a trap-gate, so
         * the error code was already pushed by the CPU */
        push    $\num    /* push interrupt number */
        jmp     isr_common_entry
    .size       isr_\num, . - isr_\num
.endm

DEFAULT_ISR 0
DEFAULT_ISR 1
DEFAULT_ISR 2
DEFAULT_ISR 3
DEFAULT_ISR 4
DEFAULT_ISR 5
DEFAULT_ISR 6
DEFAULT_ISR 7
ERROR_ISR   8
DEFAULT_ISR 9
ERROR_ISR   10
ERROR_ISR   11
ERROR_ISR   12
ERROR_ISR   13
ERROR_ISR   14
DEFAULT_ISR 15
DEFAULT_ISR 16
ERROR_ISR   17
DEFAULT_ISR 18
DEFAULT_ISR 19
DEFAULT_ISR 20
DEFAULT_ISR 21
DEFAULT_ISR 22
DEFAULT_ISR 23
DEFAULT_ISR 24
DEFAULT_ISR 25
DEFAULT_ISR 26
DEFAULT_ISR 27
DEFAULT_ISR 28
DEFAULT_ISR 29
DEFAULT_ISR 30
ERROR_ISR   31
DEFAULT_ISR 32
DEFAULT_ISR 33
DEFAULT_ISR 34
DEFAULT_ISR 35
DEFAULT_ISR 36
DEFAULT_ISR 37
DEFAULT_ISR 38
DEFAULT_ISR 39
DEFAULT_ISR 40
DEFAULT_ISR 41
DEFAULT_ISR 42
DEFAULT_ISR 43
DEFAULT_ISR 44
DEFAULT_ISR 45
DEFAULT_ISR 46
DEFAULT_ISR 47
DEFAULT_ISR 48
DEFAULT_ISR 49
DEFAULT_ISR 50
DEFAULT_ISR 51
DEFAULT_ISR 52
DEFAULT_ISR 53
DEFAULT_ISR 54
DEFAULT_ISR 55
DEFAULT_ISR 56
DEFAULT_ISR 57
DEFAULT_ISR 58
DEFAULT_ISR 59
DEFAULT_ISR 60
DEFAULT_ISR 61
DEFAULT_ISR 62
DEFAULT_ISR 63
DEFAULT_ISR 64
DEFAULT_ISR 65
DEFAULT_ISR 66
DEFAULT_ISR 67
DEFAULT_ISR 68
DEFAULT_ISR 69
DEFAULT_ISR 70
DEFAULT_ISR 71
DEFAULT_ISR 72
DEFAULT_ISR 73
DEFAULT_ISR 74
DEFAULT_ISR 75
DEFAULT_ISR 76
DEFAULT_ISR 77
DEFAULT_ISR 78
DEFAULT_ISR 79
DEFAULT_ISR 80
DEFAULT_ISR 81
DEFAULT_ISR 82
DEFAULT_ISR 83
DEFAULT_ISR 84
DEFAULT_ISR 85
DEFAULT_ISR 86
DEFAULT_ISR 87
DEFAULT_ISR 88
DEFAULT_ISR 89
DEFAULT_ISR 90
DEFAULT_ISR 91
DEFAULT_ISR 92
DEFAULT_ISR 93
DEFAULT_ISR 94
DEFAULT_ISR 95
DEFAULT_ISR 96
DEFAULT_ISR 97
DEFAULT_ISR 98
DEFAULT_ISR 99
DEFAULT_ISR 100
DEFAULT_ISR 101
DEFAULT_ISR 102
DEFAULT_ISR 103
DEFAULT_ISR 104
DEFAULT_ISR 105
DEFAULT_ISR 106
DEFAULT_ISR 107
DEFAULT_ISR 108
DEFAULT_ISR 109
DEFAULT_ISR 110
DEFAULT_ISR 111
DEFAULT_ISR 112
DEFAULT_ISR 113
DEFAULT_ISR 114
DEFAULT_ISR 115
DEFAULT_ISR 116
DEFAULT_ISR 117
DEFAULT_ISR 118
DEFAULT_ISR 119
DEFAULT_ISR 120
DEFAULT_ISR 121
DEFAULT_ISR 122
DEFAULT_ISR 123
DEFAULT_ISR 124
DEFAULT_ISR 125
DEFAULT_ISR 126
DEFAULT_ISR 127
DEFAULT_ISR 128
DEFAULT_ISR 129
DEFAULT_ISR 130
DEFAULT_ISR 131
DEFAULT_ISR 132
DEFAULT_ISR 133
DEFAULT_ISR 134
DEFAULT_ISR 135
DEFAULT_ISR 136
DEFAULT_ISR 137
DEFAULT_ISR 138
DEFAULT_ISR 139
DEFAULT_ISR 140
DEFAULT_ISR 141
DEFAULT_ISR 142
DEFAULT_ISR 143
DEFAULT_ISR 144
DEFAULT_ISR 145
DEFAULT_ISR 146
DEFAULT_ISR 147
DEFAULT_ISR 148
DEFAULT_ISR 149
DEFAULT_ISR 150
DEFAULT_ISR 151
DEFAULT_ISR 152
DEFAULT_ISR 153
DEFAULT_ISR 154
DEFAULT_ISR 155
DEFAULT_ISR 156
DEFAULT_ISR 157
DEFAULT_ISR 158
DEFAULT_ISR 159
DEFAULT_ISR 160
DEFAULT_ISR 161
DEFAULT_ISR 162
DEFAULT_ISR 163
DEFAULT_ISR 164
DEFAULT_ISR 165
DEFAULT_ISR 166
DEFAULT_ISR 167
DEFAULT_ISR 168
DEFAULT_ISR 169
DEFAULT_ISR 170
DEFAULT_ISR 171
DEFAULT_ISR 172
DEFAULT_ISR 173
DEFAULT_ISR 174
DEFAULT_ISR 175
DEFAULT_ISR 176
DEFAULT_ISR 177
DEFAULT_ISR 178
DEFAULT_ISR 179
DEFAULT_ISR 180
DEFAULT_ISR 181
DEFAULT_ISR 182
DEFAULT_ISR 183
DEFAULT_ISR 184
DEFAULT_ISR 185
DEFAULT_ISR 186
DEFAULT_ISR 187
DEFAULT_ISR 188
DEFAULT_ISR 189
DEFAULT_ISR 190
DEFAULT_ISR 191
DEFAULT_ISR 192
DEFAULT_ISR 193
DEFAULT_ISR 194
DEFAULT_ISR 195
DEFAULT_ISR 196
DEFAULT_ISR 197
DEFAULT_ISR 198
DEFAULT_ISR 199
DEFAULT_ISR 200
DEFAULT_ISR 201
DEFAULT_ISR 202
DEFAULT_ISR 203
DEFAULT_ISR 204
DEFAULT_ISR 205
DEFAULT_ISR 206
DEFAULT_ISR 207
DEFAULT_ISR 208
DEFAULT_ISR 209
DEFAULT_ISR 210
DEFAULT_ISR 211
DEFAULT_ISR 212
DEFAULT_ISR 213
DEFAULT_ISR 214
DEFAULT_ISR 215
DEFAULT_ISR 216
DEFAULT_ISR 217
DEFAULT_ISR 218
DEFAULT_ISR 219
DEFAULT_ISR 220
DEFAULT_ISR 221
DEFAULT_ISR 222
DEFAULT_ISR 223
DEFAULT_ISR 224
DEFAULT_ISR 225
DEFAULT_ISR 226
DEFAULT_ISR 227
DEFAULT_ISR 228
DEFAULT_ISR 229
DEFAULT_ISR 230
DEFAULT_ISR 231
DEFAULT_ISR 232
DEFAULT_ISR 233
DEFAULT_ISR 234
DEFAULT_ISR 235
DEFAULT_ISR 236
DEFAULT_ISR 237
DEFAULT_ISR 238
DEFAULT_ISR 239
DEFAULT_ISR 240
DEFAULT_ISR 241
DEFAULT_ISR 242
DEFAULT_ISR 243
DEFAULT_ISR 244
DEFAULT_ISR 245
DEFAULT_ISR 246
DEFAULT_ISR 247
DEFAULT_ISR 248
DEFAULT_ISR 249
DEFAULT_ISR 250
DEFAULT_ISR 251
DEFAULT_ISR 252
DEFAULT_ISR 253
DEFAULT_ISR 254
DEFAULT_ISR 255

/**
 * Common interrupt gate called from all the interrupt service routines.
 *
 * It is responsible to save the general-purpose registers and switch
 * to the rust code to handle the specific interrupt thrown
 */
.global     isr_common_entry
.type       isr_common_entry, @function
isr_common_entry:
    /* save the general purpose registers */
    push        %rax
    push        %rbx
    push        %rcx
    push        %rdx
    push        %rdi
    push        %rsi
    push        %rbp
    push        %r8
    push        %r9
    push        %r10
    push        %r11
    push        %r12
    push        %r13
    push        %r14
    push        %r15

    /* call the rust interrupt entry-point */
    mov         %rsp, %rdi
    call        interrupt_handler

    /* restore the general purpose registers */
    pop         %r15
    pop         %r14
    pop         %r13
    pop         %r12
    pop         %r11
    pop         %r10
    pop         %r9
    pop         %r8
    pop         %rbp
    pop         %rsi
    pop         %rdi
    pop         %rdx
    pop         %rcx
    pop         %rbx
    pop         %rax

    /* remove error code & interrupt number from the stack frame */
    add         $16, %rsp

    /* return to the previous caller */
    iretq
.size       isr_common_entry, . - isr_common_entry

/**
 * System call entry point
 */
.global     syscall_entry
.type       syscall_entry, @function
syscall_entry:
    /* the idea here is to read the stackpointer from the IA32_SYSENTER_ESP MSR.
     * To do that, we need to put $0x175 into %rcx.
     * rdmsr will put the result in %rax:%rdx.
     * Thus, we save %rsp,%rcx and %rdx first (%rax is not needed).
     * Afterwards, we build the stackpointer from %rax:%rdx and restore the registers
     */
    mov         %rsp, %r11
    mov         %rcx, %r9
    mov         %rdx, %rsp
    mov         $0x175, %rcx
    rdmsr
    shl         $32, %rdx
    mov         %rdx, %rcx
    mov         %rsp, %rdx
    mov         %rax, %rsp
    or          %rcx, %rsp
    mov         %r11, %rax
    mov         %r9, %rcx
    sub         $(7 * 8), %rsp /* stack layout as for interrupts/exceptions */

    /* push 0 as interrupt number, so we can recognize it */
    push        $0

    /* save the general purpose registers */
    push        %rax
    push        %rbx
    push        %rcx
    push        %rdx
    push        %rdi
    push        %rsi
    push        %rbp
    push        %r8
    push        %r9
    push        %r10
    push        %r11
    push        %r12
    push        %r13
    push        %r14
    push        %r15

    /* call the rust syscall entry-point */
    mov         %rsp, %rdi
    call        syscall_handler

    /* restore the general purpose registers */
    pop         %r15
    pop         %r14
    pop         %r13
    pop         %r12
    pop         %r11
    pop         %r10
    pop         %r9
    pop         %r8
    pop         %rbp
    pop         %rsi
    pop         %rdi
    pop         %rdx
    pop         %rcx
    pop         %rbx
    pop         %rax

    /* restore stack pointer and values */
    mov         %r11, %rsp
    mov         $0x200, %r11
    sysretq
.size       syscall_entry, . - syscall_entry