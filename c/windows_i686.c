__int64 getTimeStamp() {
    LARGE_INTEGER li;

    rdtsc;

    __asm	mov	li.LowPart, eax;
    __asm	mov	li.HighPart, edx;
    return li.QuadPart;
}
