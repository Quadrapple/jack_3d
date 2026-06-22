#define FILL_INARRAY_V2(ADDR, V_0, V_1) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1)

#define FILL_INARRAY_V3(ADDR, V_0, V_1, V_2) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1); \\
    do POKE(ADDR+2, V_2)

#define FILL_INARRAY_V4(ADDR, V_0, V_1, V_2, V_3) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1); \\
    do POKE(ADDR+2, V_2); \\
    do POKE(ADDR+3, V_3)

#define PEEK(ADDR) \
    Memory.peek(ADDR)

#define POKE(ADDR, VAL) \
    Memory.poke(ADDR, VAL)

class Cube {

//model size 8
// type = 0x01 <- cube
// v3 scaling
// v3 translation
// int outbuf3d <- where the transformed points are supposed to be saved
    function int model(int startAddr, int outbuf3d, int x, int y, int z, int xScale, int yScale, int zScale) {
        do POKE(startAddr, 1);
        FILL_INARRAY_V3(startAddr + 1, xScale, yScale, zScale);
        FILL_INARRAY_V3(startAddr + 4, x, y, z);
        do POKE(startAddr + 7, outbuf3d);

        return startAddr + 8;
    }

    //cube face: <- size 7
    //4 indices for edges
    //then a 3d normal
    function int faces(int startAddr) {
        var int stride;
        let stride = 7;

        FILL_INARRAY_V4(startAddr, 0, 1, 2, 3); //edges
        FILL_INARRAY_V3(startAddr + 4, 0, -1, 0);   //normal
        let startAddr = startAddr + stride;

        FILL_INARRAY_V4(startAddr, 0, 7, 8, 4);
        FILL_INARRAY_V3(startAddr + 4, 0, 0, -1);
        let startAddr = startAddr + stride;

        FILL_INARRAY_V4(startAddr, 3, 4, 11, 5);
        FILL_INARRAY_V3(startAddr + 4, -1, 0, 0);
        let startAddr = startAddr + stride;

        FILL_INARRAY_V4(startAddr, 2, 5, 10, 6);
        FILL_INARRAY_V3(startAddr + 4, 0, 0, 1);
        let startAddr = startAddr + stride;

        FILL_INARRAY_V4(startAddr, 1, 6, 9, 7);
        FILL_INARRAY_V3(startAddr + 4, 1, 0, 0);
        let startAddr = startAddr + stride;

        FILL_INARRAY_V4(startAddr, 8, 9, 10, 11);
        FILL_INARRAY_V3(startAddr + 4, 0, 1, 0);
        let startAddr = startAddr + stride;

        return startAddr;
    }

    function int edges(int startAddr) {
        FILL_INARRAY_V2(startAddr, 0, 1);      //0
        FILL_INARRAY_V2(startAddr + 2, 1, 3);  //1
        FILL_INARRAY_V2(startAddr + 4, 3, 2);  //2
        FILL_INARRAY_V2(startAddr + 6, 2, 0);  //3

        FILL_INARRAY_V2(startAddr + 8, 0, 4);  //4
        FILL_INARRAY_V2(startAddr + 10, 2, 6); //5
        FILL_INARRAY_V2(startAddr + 12, 3, 7); //6
        FILL_INARRAY_V2(startAddr + 14, 1, 5); //7

        FILL_INARRAY_V2(startAddr + 16, 4, 5); //8
        FILL_INARRAY_V2(startAddr + 18, 5, 7); //9
        FILL_INARRAY_V2(startAddr + 20, 7, 6); //10
        FILL_INARRAY_V2(startAddr + 22, 6, 4); //11

        return startAddr + 24;
    }

    function int vertices(int startAddr) {
        FILL_INARRAY_V3(startAddr, 0, 0, 0);    //0
        FILL_INARRAY_V3(startAddr+3, 1, 0, 0);  //1
        FILL_INARRAY_V3(startAddr+6, 0, 0, 1);  //2
        FILL_INARRAY_V3(startAddr+9, 1, 0, 1);  //3

        FILL_INARRAY_V3(startAddr+12, 0, 1, 0); //4
        FILL_INARRAY_V3(startAddr+15, 1, 1, 0); //5
        FILL_INARRAY_V3(startAddr+18, 0, 1, 1); //6
        FILL_INARRAY_V3(startAddr+21, 1, 1, 1); //7

        return startAddr + 24;
    }
}
