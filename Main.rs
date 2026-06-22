
// Configuration:
//
// Changes the projection to top-down orthogonal, breaks portal visibility logic
// #define TOPDOWN_VIEW
//
// SPACE up, COMMA down
// #define FLIGHT
// 
// Extremely slow on the VME, use the browser emulator
   #define HIGH_PRECISION_ARITHMETIC
//
// Draws an outline around portals
   #define DRAW_PORTAL_EDGES
//
//
// Debug:
#define __FRAME_DEBUG
// #define __POINT_DEBUG
// #define __POS_DEBUG
// #define __ROTMAT_DEBUG

#define DECL_M2X2(NAME, TYPE) \\
    TYPE int NAME##_2x2m_00; \\
    TYPE int NAME##_2x2m_01; \\
    TYPE int NAME##_2x2m_10; \\
    TYPE int NAME##_2x2m_11; \\

#define DECL_M3X3(NAME, TYPE) \\
    TYPE int NAME##_3x3m_00; \\
    TYPE int NAME##_3x3m_01; \\
    TYPE int NAME##_3x3m_02; \\
    TYPE int NAME##_3x3m_10; \\
    TYPE int NAME##_3x3m_11; \\
    TYPE int NAME##_3x3m_12; \\
    TYPE int NAME##_3x3m_20; \\
    TYPE int NAME##_3x3m_21; \\
    TYPE int NAME##_3x3m_22 

#define DECL_V2(NAME, TYPE) \\
    TYPE int NAME##_2v_0; \\
    TYPE int NAME##_2v_1

#define DECL_V3(NAME, TYPE) \\
    TYPE int NAME##_3v_0; \\
    TYPE int NAME##_3v_1; \\
    TYPE int NAME##_3v_2

#define DECL_V4(NAME, TYPE) \\
    TYPE int NAME##_4v_0; \\
    TYPE int NAME##_4v_1; \\
    TYPE int NAME##_4v_2; \\
    TYPE int NAME##_4v_3

#define M2X2(NAME, I, J) \
    NAME##_2x2m_##I##J

#define M3X3(NAME, I, J) \
    NAME##_3x3m_##I##J

#define V2(NAME, I) \
    NAME##_2v_##I

#define V3(NAME, I) \
    NAME##_3v_##I

#define V4(NAME, I) \
    NAME##_4v_##I

#define FILL_V2(NAME, V_0, V_1) \
    let V2(NAME, 0) = V_0; \\
    let V2(NAME, 1) = V_1

#define FILL_V2_ADDR(NAME, ADDR) \
    let V2(NAME, 0) = PEEK(ADDR); \\
    let V2(NAME, 1) = PEEK(ADDR+1)

#define FILL_V3(NAME, V_0, V_1, V_2) \
    let V3(NAME, 0) = V_0; \\
    let V3(NAME, 1) = V_1; \\
    let V3(NAME, 2) = V_2

#define FILL_V3_ADDR(NAME, ADDR) \
    let V3(NAME, 0) = PEEK(ADDR); \\
    let V3(NAME, 1) = PEEK(ADDR+1); \\
    let V3(NAME, 2) = PEEK(ADDR+2)

#define FILL_V4(NAME, V_0, V_1, V_2, V_3) \
    let V4(NAME, 0) = V_0; \\
    let V4(NAME, 1) = V_1; \\
    let V4(NAME, 2) = V_2; \\
    let V4(NAME, 3) = V_3

#define FILL_M3X3_ROW(NAME, I, V_0, V_1, V_2) \
    let M3X3(NAME, I, 0) = V_0; \\
    let M3X3(NAME, I, 1) = V_1; \\
    let M3X3(NAME, I, 2) = V_2

#define FILL_INARRAY_V4(ADDR, V_0, V_1, V_2, V_3) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1); \\
    do POKE(ADDR+2, V_2); \\
    do POKE(ADDR+3, V_3)

#define FILL_INARRAY_V3(ADDR, V_0, V_1, V_2) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1); \\
    do POKE(ADDR+2, V_2)

#define FILL_INARRAY_V2(ADDR, V_0, V_1) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1)

//must have buf0..3 declared;
//iffy for negatives, still works ok
//second division is by 128 because otherwise i'd need to multiply the buf0 * buf1 term by 2
#define MULTIPLY_15_15(IN0, IN1, O) \
    let buf0 = IN0 / 256; \\
    let buf1 = IN1 / 128; \\
\\
    let buf2 = IN0 & 255;\\
    let buf3 = IN1 & 255;\\
\\
    let O = (buf0 * buf1) \\
    + ((buf0 * buf3) / 256)\\
    + ((buf1 * buf2) / 256)

//32 bit multiplication
#define MULTIPLY_FULLPRECISION(IN0, IN1, Oh, Ol) \
    let buf1_l = IN1 & 255;\\
    let buf1_h = (IN1 - buf1_l) / 256; \\
\\
    let buf0_l = IN0 & 255;\\
    let buf0_h = (IN0 - buf0_l) / 256;\\
\\
    let buf_lh = buf0_l * buf1_h; \\
    let buf_hl = buf1_l * buf0_h; \\
    let buf_ll = buf0_l * buf1_l; \\
\\
    let Oh = (buf0_h * buf1_h) + ((buf_lh - (buf_lh & 255)) / 256) + ((buf_hl - (buf_hl & 255)) / 256); \\
    let Ol = buf0_l * buf1_l; \\
\\
    let tmp =  ((buf_lh & 255) * 256);\\
\\
    if( (Ol < 0) & (tmp < 0) ) { \\
        let Oh = Oh + 1; \\
        let Ol = Ol + tmp; \\
    } else { \\
        if( (Ol < 0) = (tmp < 0) ) { \\
            let Ol = Ol + tmp; \\
        } else { \\
            let Ol = Ol + tmp; \\
            if(Ol < 0){ \\
            } else {\\
                let Oh = Oh + 1; \\
            }\\
        }\\
    }\\
    let tmp =  ((buf_hl & 255) * 256);\\
    if( (Ol < 0) & (tmp < 0) ) { \\
        let Oh = Oh + 1; \\
        let Ol = Ol + tmp; \\
    } else { \\
        if( (Ol < 0) = (tmp < 0) ) { \\
            let Ol = Ol + tmp; \\
        } else { \\
            let Ol = Ol + tmp; \\
            if(Ol < 0){ \\
            } else {\\
                let Oh = Oh + 1; \\
            }\\
        }\\
    }

//32 bit addition
#define ADD_FULLPRECISION(IN0_l, IN0_h, IN1_l, IN1_h, Oh, Ol) \
    let Oh = IN0_h + IN1_h; \\
    let Ol = IN0_l + IN1_l; \\
    if((IN0_l < 0) & (IN1_l < 0)) { \\
        let Oh = Oh + 1; \\
    } else { \\
        if(~((IN0_l < 0) = (IN1_l < 0))) { \\
            if(~(Ol < 0)) { \\
                let Oh = Oh + 1; \\
            } \\
        } \\
    }

//must have buf0..3 declared;
//the same logic as MULTIPLY_15_15
#define ADD_MULTIPLIED_15_15(IN0, IN1, ACC) \
    let buf0 = IN0 / 256; \\
    let buf1 = IN1 / 128; \\
\\
    let buf2 = IN0 & 255;\\
    let buf3 = IN1 & 255;\\
\\
    let ACC = ACC + (buf0 * buf1) \\
    + ((buf0 * buf3) / 256)\\
    + ((buf1 * buf2) / 256)

//output scaled signed 2^15
#define MULTIPLY_OFFSET_7_15(IN0_7, IN1_7h, IN1_7l, O) \
    let O = (IN0_7 * IN1_7h) + ((IN0_7 * IN1_7l) / 256)


//output scaled signed 2^15
#define ADD_MULTIPLIED_OFFSET_7_15(IN0_7, IN1_7h, IN1_7l, ACC) \
    let ACC = ACC + (IN0_7 * IN1_7h) + ((IN0_7 * IN1_7l) / 256)

//output scaled signed 2^15
#define MULTIPLY_7_10(IN0_7, IN1_10, O) \
    let buf0 = IN0_7; \\
    let buf1 = IN1_10 / 4; \
\\
    let buf3 = IN1_10 & 7;\\
\\
    let O = (buf0 * buf1) \\
    + ((buf0 * buf3) / 8)


//output scaled signed 2^15
#define ADD_MULTIPLIED_7_10(IN0_7, IN1_10, ACC) \
    let buf0 = IN0_7; \\
    let buf1 = IN1_10 / 4; \\
\\
    let buf3 = IN1_10 & 7;\\
\\
    let ACC = ACC + (buf0 * buf1) \\
    + ((buf0 * buf3) / 8)

//output scaled signed 2^15
#define MULTIPLY_7_15(IN0_7, IN1_15, O) \
    let O = (IN0_7 * (IN1_15 / 256)) + ((IN0_7 * (IN1_15 & 255)) / 256)

//output scaled signed 2^15
#define MULTIPLY_7_7(IN0_7, IN1_7, O) \
    let O = (IN0_7 * IN1_7) 

//output scaled signed 2^7
#define MULTIPLY_7_7_O7(IN0_7, IN1_7, O) \
    let O = (IN0_7 * IN1_7) / 128

#define MULTIPLY_7_7_O7_INLINE(IN0_7, IN1_7) \
    (IN0_7 * IN1_7) / 128

#define MULTIPLY_7_7_O14_INLINE(IN0_7, IN1_7) \
    (IN0_7 * IN1_7) / 2

//output scaled signed 2^15
#define ADD_MULTIPLIED_7_7(IN0_7, IN1_7, ACC) \
    let ACC = ACC + (IN0_7 * IN1_7) 

#define PRINT_VAL(VSTR, VSTR_LEN, VAL, Y, X) \
    do Output.moveCursor(Y, X); \\
    do Output.printString(VSTR); \\
    do Output.printInt(VAL)

#define PEEK(ADDR) \
    Memory.peek(ADDR)

#define POKE(ADDR, VAL) \
    Memory.poke(ADDR, VAL)

#define MDL_SCALE(ADDR, IND) \
    PEEK(ADDR + 1 + IND)

#define MDL_POS(ADDR, IND) \
    PEEK(ADDR + 4 + IND)

#define MDL_P3D(ADDR) \
    PEEK(ADDR + 7)

#define ROOM_MDL_COUNT(ADDR) \
    PEEK(ADDR + 6)

#define ROOM_MDLS(ADDR) \
    (ADDR + 7)

#define ROOM_XMIN(ADDR) \
    PEEK(ADDR)
#define ROOM_XMAX(ADDR) \
    PEEK(ADDR+1)
#define ROOM_YMIN(ADDR) \
    PEEK(ADDR+2)
#define ROOM_YMAX(ADDR) \
    PEEK(ADDR+3)
#define ROOM_ZMIN(ADDR) \
    PEEK(ADDR+4)
#define ROOM_ZMAX(ADDR) \
    PEEK(ADDR+5)

#define PRT_P3D(ADDR) \
    PEEK(ADDR+10)
#define PRT_ROOM(ADDR) \
    PEEK(ADDR+9)

//right now i only have 1 model
//so it's fine to hardcode everything

//stores edge buffer for backface culling
//same size as edge
#define ADDR_EF 10000
#define ADDR_EF_END 10024

//stores edges
#define ADDR_E 11000
#define ADDR_E_END 11024

//stores the edges of faces
//nr of edges/face is undefined by itself
#define ADDR_F 12000
#define ADDR_F_END 12042

//room <- size 8 + model_amount + PORTAL_STRIDE*portal_amount
// int x_min, x_max, y_min, y_max, z_min, z_max <- room bounding box
// int model_amount
// int mdl1, mdl2, mdl3,... <- array of addresses of models
// 
// int portal_amount
// portal prt1, prt2,... <- a portal is stored directly as a sequence of 11 ints, not as an address
// prt: v3 topright, v3 bottomleft, v3 normal, int other_room_addr, int output3d <- size 11
#define ADDR_ROOM 9000

//address of the model array
//model memory layout <- stride 1+3+3+1=8
// int model type
// v3 scaling
// v3 translation
// int outbuf3d <- location of transformed points
#define ADDR_MDL 13000
#define ADDR_MDL_END 13032

#define MODEL_SIZE 8

//stores buffered 2d edges as 4 ints (2 points)
#define ADDR_P2D 14024
#define ADDR_P2D_END 14168

//stores points transformed by the viewMat
//to later be processed into edges
#define ADDR_P3D_R 15000
// #define ADDR_P3D_R_END 15096

//stores model prototype points
#define ADDR_P3D_CUBE 16168
#define ADDR_P3D_CUBE_END 16384

//maybe redundant
#define Y_MAX 253
#define Y_MIN 2
#define X_MAX 510
#define X_MIN 2

//forward is -z
//everything with z > NEAR_PLANE is supposed to be behind and thus clipped
#define NEAR_PLANE -128

//readability aids
#define CUBE_VERTEX_COUNT 8
#define CUBE_EDGES_COUNT 12
#define CUBE_FACES_COUNT 6

#define EYE_LEVEL 2

class Main {
    //position in world space + fractional part for consistent directional motion
    //guess the addresses: 16, 17, 18 and 19, 20, 21
    DECL_V3(pos, static);
    DECL_V3(posFrac, static);
    DECL_V3(velocity, static);

    static int room_addr_end;
    static int current_room;

    #ifdef __POS_DEBUG
        static String room_str;

        static String x_str;
        static String y_str;
        static String z_str;
    #endif

    #ifdef __POINT_DEBUG
        static String x_str;
        static String y_str;
        static String z_str;
        static String xt_str;
        static String yt_str;
        static String zt_str;
    #endif

    //functions as a pointer to the draw buffer during the buffering pass
    //and as the end of the draw buffer during the draw pass
    static int drawBuf;

    //representation: unsigned 0-255
    //remove 128 wherever neccessary
    static int pitch, yaw;

    //useful to not waste time passing these arguments
    static int x1, y1, z1, x2, y2, z2;

    //scaled signed 2^7
    DECL_M3X3(view, static);

    //signed 2^7
    static int sinYaw, cosYaw;

    //SCALING signed 2^7
    static Array sinTable;

    static String blank_str;
    function void main() {
        var String padding;
        var int x_t, y_t, z_t;

        #ifdef __POS_DEBUG
            var String xf_str;
            var String yf_str;
            var String zf_str;
            var String xv_str;
            var String yv_str;
            var String zv_str;
        #endif

        #ifdef __FRAME_DEBUG
            var String frame_str;
            var int frame;
        #endif

        #ifdef __ROTMAT_DEBUG
            var String yaw_str;
            var String pitch_str;
            var String sin_str;
            var String cos_str;
            var String padding;

        //i apparently cant use literals and have to deal w ts
        //cause otherwise it goes 'heap limit reached'
            let sin_str = "SIN: ";
            let cos_str = "COS: ";
            let yaw_str = "YAW: ";
            let pitch_str = "PIT: ";
        #endif
        let blank_str = "_: ";

        #ifdef __POINT_DEBUG
            let x_str = "X: ";
            let xt_str = "XT: ";
            let y_str = "Y: ";
            let yt_str = "YT: ";
            let z_str = "Z: ";
            let zt_str = "ZT: ";
        #endif

        #ifdef __POS_DEBUG
            let room_str = "ROOM: ";
            let x_str = "X: ";
            let y_str = "Y: ";
            let z_str = "Z: ";
            let xf_str= "XF:";
            let yf_str= "YF:";
            let zf_str= "ZF:";
            let xv_str= "XV:";
            let yv_str= "YV:";
            let zv_str= "ZV:";
        #endif

        #ifdef __FRAME_DEBUG
            let frame_str = "FRAME: ";
            let frame = 0;
        #endif
        let padding = "        ";

        FILL_V3(pos, 0, 0, 5);
        FILL_V3(posFrac, 128, 128, 128);
        FILL_V3(velocity, 0, 0, 0);

        //make sure the spot in the view matrix that's supposed to be zero is zero
        //cause it's never getting updated again
        let M3X3(view, 0, 1) = 0;

        //set pitch, yaw to zero
        //they are stored as if they were signed 8-bit integers, so 128 is 0;
        let pitch = 128;
        let yaw = 128;
        do Main.fillPoints();
        do Main.fillSinTable();

        //game loop
        while( -1 ) {
            do Main.calcViewMat();
            do Main.pollKeyboard();

            do Main.getCurrentRoom();

            //buffer transformed points
            do Main.transformPoints();

            //buffer edges to the draw buffer
            do Main.bufferEdges();

            //clear pass
            do Screen.clearScreen();

            #ifdef __FRAME_DEBUG
                PRINT_VAL(frame_str, 6, frame, 0, 32);
                let frame = frame + 1;
            #endif

            #ifdef __POS_DEBUG
                PRINT_VAL(x_str, 4, V3(pos, 0), 0, 0);
                PRINT_VAL(xf_str, 4, V3(posFrac, 0), 1, 0);
                PRINT_VAL(xv_str, 4, V3(velocity, 0), 2, 0);

                PRINT_VAL(y_str, 4, V3(pos, 1), 0, 10);
                PRINT_VAL(yf_str, 4, V3(posFrac, 1), 1, 10);
                PRINT_VAL(yv_str, 4, V3(velocity, 1), 2, 10);

                PRINT_VAL(z_str, 4, V3(pos, 2), 0, 20);
                PRINT_VAL(zf_str, 4, V3(posFrac, 2), 1, 20);
                PRINT_VAL(zv_str, 4, V3(velocity, 2), 2, 20);

                PRINT_VAL(room_str, 7, current_room, 3, 0);
            #endif

            #ifdef __ROTMAT_DEBUG
                PRINT_VAL(yaw_str, 5, yaw, 0, 0);
                PRINT_VAL(sin_str, 5, Main.sin(yaw), 1, 0);
                PRINT_VAL(cos_str, 5, Main.cos(yaw), 2, 0);

                PRINT_VAL(pitch_str, 5, pitch, 0, 20);
                PRINT_VAL(sin_str, 5, Main.sin(pitch), 1, 20);
                PRINT_VAL(cos_str, 5, Main.cos(pitch), 2, 20);

                PRINT_VAL(blank_str, 4, M3X3(view, 0, 0), 8, 0);
                PRINT_VAL(blank_str, 4, M3X3(view, 0, 1), 8, 9);
                PRINT_VAL(blank_str, 4, M3X3(view, 0, 2), 8, 18);
                PRINT_VAL(blank_str, 4, M3X3(view, 1, 0), 9, 0);
                PRINT_VAL(blank_str, 4, M3X3(view, 1, 1), 9, 9);
                PRINT_VAL(blank_str, 4, M3X3(view, 1, 2), 9, 18);
                PRINT_VAL(blank_str, 4, M3X3(view, 2, 0), 10, 0);
                PRINT_VAL(blank_str, 4, M3X3(view, 2, 1), 10, 9);
                PRINT_VAL(blank_str, 4, M3X3(view, 2, 2), 10, 18);
            #endif

            #ifdef __POINT_DEBUG
                PRINT_VAL(x_str, 4, x1, 5, 0);
                PRINT_VAL(x_str, 4, x2, 6, 0);

                PRINT_VAL(y_str, 4, y1, 5, 12);
                PRINT_VAL(y_str, 4, y2, 6, 12);

                PRINT_VAL(z_str, 4, z1, 5, 24);
                PRINT_VAL(z_str, 4, z2, 6, 24);
            #endif

            //draw pass
            //render out all the edges stored in the draw buffer
            do Main.draw();
        }
        return;
    }

    #define PORTAL_STRIDE 11

    function void getCurrentRoom() {
        var int room_addr, room_mdl_count, room_prt_count;
        let room_addr = ADDR_ROOM;
        let current_room = 0;

        while(room_addr < room_addr_end) {
            if((V3(pos, 0) < (ROOM_XMAX(room_addr) + 1)) & (V3(pos, 0) > (ROOM_XMIN(room_addr) - 1)) &
                (V3(pos, 1) < (ROOM_YMAX(room_addr) + 1)) & (V3(pos, 1) > (ROOM_YMIN(room_addr) - 1)) & 
                (V3(pos, 2) < (ROOM_ZMAX(room_addr) + 1)) & (V3(pos, 2) > (ROOM_ZMIN(room_addr) - 1))) {

                let current_room = room_addr;
                return;
            }

            //get next room
            let room_mdl_count = ROOM_MDL_COUNT(room_addr);
            let room_addr = room_addr + 7 + room_mdl_count; // <- addr now at prt_count

            let room_prt_count = PEEK(room_addr);
            let room_addr = room_addr + 1 + (room_prt_count * PORTAL_STRIDE);
        }
        //set to 0 if none found
        let current_room = 0;
        return;
    }

    function void transformPoints() {
        var int room_addr, prt_addr, room_prt_addr_end, room_mdl_count, room_prt_count;

        if(current_room = 0) {
            let room_addr = ADDR_ROOM;
            while(room_addr < room_addr_end) {
//              do Main.breakpoint();
                do Main.transformPointsInRoom(room_addr);

                //get next room
                let room_mdl_count = ROOM_MDL_COUNT(room_addr);
                let room_addr = room_addr + 7 + room_mdl_count; // <- addr now at prt_count

                let room_prt_count = PEEK(room_addr);
                let room_addr = room_addr + 1 + (room_prt_count * PORTAL_STRIDE);
            }

        } else {
            let room_addr = current_room;
//          do Main.breakpoint();
            do Main.transformPointsInRoom(room_addr);

            let room_mdl_count = ROOM_MDL_COUNT(room_addr);
            let room_addr = room_addr + 7 + room_mdl_count; // <- room_addr now at prt_count

            let room_prt_count = PEEK(room_addr);
            let prt_addr = room_addr + 1;
            let room_prt_addr_end = prt_addr + (room_prt_count * PORTAL_STRIDE);

//          do Output.moveCursor(10, 0);
//          do Output.printInt(room_prt_addr_end);
//          do Output.println();
//          do Output.printInt(room_prt_count);
//          do Output.println();
//          do Output.printInt(prt_addr);
//          do Output.println();

            do Main.breakpoint();
            while(prt_addr < room_prt_addr_end) {
                do Main.transformPointsInRoom(PRT_ROOM(prt_addr));
                let prt_addr = prt_addr + PORTAL_STRIDE;
            }
        }
        return;
    }


    #define VIEW_POSFRAC_TRANSFORM(X_IN, Y_IN, Z_IN, X_OUT, Y_OUT, Z_OUT) \
        MULTIPLY_OFFSET_7_15(M3X3(view, 2, 0), X_IN, V3(posFrac, 0), Z_OUT);\\
        ADD_MULTIPLIED_7_7(M3X3(view, 2, 1), Y_IN, Z_OUT); \\
        ADD_MULTIPLIED_OFFSET_7_15(M3X3(view, 2, 2), Z_IN, V3(posFrac, 2), Z_OUT); \\
\\
        MULTIPLY_OFFSET_7_15(M3X3(view, 0, 0), X_IN, V3(posFrac, 0), X_OUT); \\
        ADD_MULTIPLIED_OFFSET_7_15(M3X3(view, 0, 2), Z_IN, V3(posFrac,2), X_OUT); \\
\\
        MULTIPLY_OFFSET_7_15(M3X3(view, 1, 0), X_IN, V3(posFrac, 0), Y_OUT); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 1, 1), Y_IN, Y_OUT);\\
        ADD_MULTIPLIED_OFFSET_7_15(M3X3(view, 1, 2), Z_IN, V3(posFrac, 2), Y_OUT)

    function void transformPointsInRoom(int room_addr) {
        var int room_mdl_addr, room_mdls_end, room_prts_end;
        var int room_prt_count;
        var int mdl_addr, prt_addr;
        var int model_prototype, model_prototype_end, points3d_addr_local;

        var int x, y, z;
        var int x_t, y_t, z_t;
        var int x_offset, y_offset, z_offset; 
        var int x_scale, y_scale, z_scale;

        var int x_tr, y_tr, z_tr, x_bl, y_bl, z_bl;

        let room_mdl_addr = ROOM_MDLS(room_addr);
        let room_mdls_end = room_mdl_addr + ROOM_MDL_COUNT(room_addr);

//      do Output.printInt(room_addr);
//      do Output.println();
//      do Output.printInt(room_mdl_addr);
//      do Output.println();
//      do Output.printInt(room_mdls_end);
//      do Output.println();

        //loop over all models in room
        while(room_mdl_addr < room_mdls_end) {
            let mdl_addr = PEEK(room_mdl_addr);

            //theres only the cube right now
            let model_prototype = ADDR_P3D_CUBE;
            let model_prototype_end = model_prototype + (3*CUBE_VERTEX_COUNT); 

//          do Output.printInt(mdl_addr);
//          do Output.println();
//          do Main.breakpoint();
            let points3d_addr_local = MDL_P3D(mdl_addr);

            let x_scale = MDL_SCALE(mdl_addr, 0);
            let y_scale = MDL_SCALE(mdl_addr, 1);
            let z_scale = MDL_SCALE(mdl_addr, 2);
            let x_offset = MDL_POS(mdl_addr, 0);
            let y_offset = MDL_POS(mdl_addr, 1);
            let z_offset = MDL_POS(mdl_addr, 2);

            //3d view transform
            while(model_prototype < model_prototype_end) {
                let x = ((x_scale * (PEEK(model_prototype)) - V3(pos, 0))) + x_offset;
                let y = ((y_scale * (PEEK(model_prototype + 1)) - V3(pos, 1))) + y_offset;
                let z = ((z_scale * (PEEK(model_prototype + 2)) - V3(pos, 2))) + z_offset;

                VIEW_POSFRAC_TRANSFORM(x, y, z, x_t, y_t, z_t);

                FILL_INARRAY_V3(points3d_addr_local, x_t / 2, y_t / 2, z_t / 2);

                let model_prototype = model_prototype + 3;
                let points3d_addr_local = points3d_addr_local + 3;
            }
            let room_mdl_addr = room_mdl_addr + 1;
        }

        let room_prt_count = PEEK(room_mdl_addr);
        let prt_addr = room_mdl_addr + 1;
        let room_prts_end = prt_addr + (room_prt_count * PORTAL_STRIDE);  

        //loop over all portals in room
        //and transform their bounds
        while(prt_addr < room_prts_end) {
            let points3d_addr_local = PRT_P3D(prt_addr);

            let x_tr = PEEK(prt_addr);
            let y_tr = PEEK(prt_addr + 1);
            let z_tr = PEEK(prt_addr + 2);
            let x_bl = PEEK(prt_addr + 3);
            let y_bl = PEEK(prt_addr + 4);
            let z_bl = PEEK(prt_addr + 5);

            //3d view transform
            //topright
            let x = x_tr - V3(pos, 0);
            let y = y_tr - V3(pos, 1);
            let z = z_tr - V3(pos, 2);
            VIEW_POSFRAC_TRANSFORM(x, y, z, x_t, y_t, z_t);
            FILL_INARRAY_V3(points3d_addr_local, x_t / 2, y_t / 2, z_t / 2);
            let points3d_addr_local = points3d_addr_local + 3;

            //bottomright
            let y = y_bl - V3(pos, 1);
            VIEW_POSFRAC_TRANSFORM(x, y, z, x_t, y_t, z_t);
            FILL_INARRAY_V3(points3d_addr_local, x_t / 2, y_t / 2, z_t / 2);
            let points3d_addr_local = points3d_addr_local + 3;

            //bottomleft
            let x = x_bl - V3(pos, 0);
            let z = z_bl - V3(pos, 2);
            VIEW_POSFRAC_TRANSFORM(x, y, z, x_t, y_t, z_t);
            FILL_INARRAY_V3(points3d_addr_local, x_t / 2, y_t / 2, z_t / 2);
            let points3d_addr_local = points3d_addr_local + 3;

            //topleft
            let y = y_tr - V3(pos, 1);
            VIEW_POSFRAC_TRANSFORM(x, y, z, x_t, y_t, z_t);
            FILL_INARRAY_V3(points3d_addr_local, x_t / 2, y_t / 2, z_t / 2);

            let prt_addr = prt_addr + PORTAL_STRIDE;
        }
        return;
    }

    #define CLAMP(VAL, MIN, MAX) \
        let VAL = Math.max(VAL, MIN); \\
        let VAL = Math.min(VAL, MAX)


    function void bufferEdges() {
        var int room_addr, prt_addr, room_prt_addr_end, room_mdl_count, room_prt_count;

        if(current_room = 0) {
            let room_addr = ADDR_ROOM;
            while(room_addr < room_addr_end) {
                do Main.bufferEdgesInRoom(room_addr);

                //get next room
                let room_mdl_count = ROOM_MDL_COUNT(room_addr);
                let room_addr = room_addr + 7 + room_mdl_count; // <- room_addr now at prt_count

                let room_prt_count = PEEK(room_addr);
                let room_addr = room_addr + 1 + (room_prt_count * PORTAL_STRIDE);
            }

        } else {
            let room_addr = current_room;
            do Main.bufferEdgesInRoom(room_addr);

            let room_mdl_count = ROOM_MDL_COUNT(room_addr);
            let room_addr = room_addr + 7 + room_mdl_count; // <- room_addr now at prt_count

            let room_prt_count = PEEK(room_addr);
            let prt_addr = room_addr + 1;
            let room_prt_addr_end = prt_addr + (room_prt_count * PORTAL_STRIDE);

            while(prt_addr < room_prt_addr_end) {
                do Main.breakpoint();
                do Main.bufferEdgesInRoomPortaled(prt_addr);
                let prt_addr = prt_addr + PORTAL_STRIDE;
            }
        }

        return;
    }


    #define VIEW_TRANSFORM(X_IN, Y_IN, Z_IN, X_OUT, Y_OUT, Z_OUT) \
        MULTIPLY_7_7(M3X3(view, 0, 0), x_normal,  x2); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 0, 2), z_normal,  x2); \\
\\
        MULTIPLY_7_7(M3X3(view, 1, 0), x_normal,  y2); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 1, 1), y_normal, y2); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 1, 2), z_normal,  y2); \\
\\
        MULTIPLY_7_7(M3X3(view, 2, 0), x_normal,  z2); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 2, 1), y_normal, z2); \\
        ADD_MULTIPLIED_7_7(M3X3(view, 2, 2), z_normal,  z2)

    #ifndef HIGH_PRECISION_ARITHMETIC
    #define PERSPECTIVE_DIVISION(N, D, OUT) \
        let OUT = (4*N) / (D / 32);
    #endif

    #ifdef HIGH_PRECISION_ARITHMETIC
    #define PERSPECTIVE_DIVISION(N, D, OUT) \
        if(N < 0) { \\
            DIVIDE_7_POSITIVE(-N, -D, OUT); \\
        } else { \\
            DIVIDE_7_POSITIVE(N, -D, OUT); \\
            let OUT = -OUT; \\
        }

    #define DIVIDE_7_POSITIVE(N, D, OUT) \
        let OUT = 0; \\
        let denominator = D; \\
        let remainder = 2 * N; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf; \\
        let remainder = remainder - (divbuf * denominator); \\
\\
        let remainder = 2 * remainder; \\
        let divbuf = remainder / denominator; \\
        let OUT = (2*OUT) + divbuf


    #define DIVIDE_7(N, D, OUT) \
        if(N < 0) { \\
            if(D < 0) { \\
                DIVIDE_7_POSITIVE(-N, -D, OUT); \\
            } else { \\
                DIVIDE_7_POSITIVE(-N, D, OUT); \\
                let OUT = -OUT; \\
            } \\
        } else { \\
            if(D < 0) { \\
                DIVIDE_7_POSITIVE(N, -D, OUT); \\
                let OUT = -OUT; \\
            } else { \\
                DIVIDE_7_POSITIVE(N, D, OUT); \\
            } \\
        }
    #endif

    //project and clamp the values to screenspace to compensate for numerical inaccuracy
    //everything that's not supposed to be drawn should habe been discarded by this point
    #ifndef TOPDOWN_VIEW
    #define PROJECT_EDGE(X1, Y1, Z1, X2, Y2, Z2) \
        let Z1 = Math.min(Z1, -32);\\
        let Z2 = Math.min(Z2, -32);\\
\\
        PERSPECTIVE_DIVISION(X1, Z1, x1_draw) \\
        PERSPECTIVE_DIVISION(Y1, Z1, y1_draw) \\
        PERSPECTIVE_DIVISION(X2, Z2, x2_draw) \\
        PERSPECTIVE_DIVISION(Y2, Z2, y2_draw) \\
        let y2_draw = y2_draw

    #endif

    #ifdef TOPDOWN_VIEW
    #define PROJECT_EDGE(X1, Y1, Z1, X2, Y2, Z2) \
        let x1_draw = (   (-X1) / (6) );\\
        let y1_draw = (   (-Z1) / (6) );\\
        let x2_draw = (   (-X2) / (6) );\\
        let y2_draw = (   (-Z2) / (6) )

    #endif

    #define CLIP_PROJECT_EDGE(X1, Y1, Z1, X2, Y2, Z2) \
        let x1 = X1; \\
        let y1 = Y1; \\
        let z1 = Z1; \\
    \\
        let x2 = X2;\\
        let y2 = Y2;\\
        let z2 = Z2;\\
\\
        let res = Main.line();\\
        if(res) {\\
            PROJECT_EDGE(x1, y1, z1, x2, y2, z2);\\
        } else {\\
            let x1_draw = 0;\\
            let y1_draw = 0;\\
            let x2_draw = 0;\\
            let y2_draw = 0;\\
        }

    #define CLIP_PROJECT_BUFFER_EDGE(X1, Y1, Z1, X2, Y2, Z2) \
        let x1 = X1; \\
        let y1 = Y1; \\
        let z1 = Z1; \\
    \\
        let x2 = X2;\\
        let y2 = Y2;\\
        let z2 = Z2;\\
\\
        let res = Main.line();\\
        if(res) {\\
            PROJECT_EDGE(x1, y1, z1, x2, y2, z2);\\
            let x1_draw = x1_draw + 256;\\
            let y1_draw = y1_draw + 128;\\
            let x2_draw = x2_draw + 256;\\
            let y2_draw = y2_draw + 128;\\
            \\
            CLAMP(x1_draw, 1, 511);\\
            CLAMP(y1_draw, 1, 255);\\
            CLAMP(x2_draw, 1, 511);\\
            CLAMP(y2_draw, 1, 255);\\
\\
            FILL_INARRAY_V4(drawBuf, x1_draw, y1_draw, x2_draw, y2_draw);\\
            let drawBuf = drawBuf + 4;\\
        } else { \\
            let x1_draw = 0;\\
            let y1_draw = 0;\\
            let x2_draw = 0;\\
            let y2_draw = 0;\\
        }

    function void bufferEdgesInRoom(int room_addr) {
        var int room_mdl_addr, room_mdls_end;
        var int mdl_addr, points_addr, edge_addr, face_addr, face_edge_map_addr;

        var int x1_draw, y1_draw, x2_draw, y2_draw;
        var int x_normal, y_normal, z_normal;
        var int remainder, divbuf, denominator;

        //edge addresses
        var int src, dst;

        //stores draw edge flag
        var bool res;

        //for 32bit
//      var int buf0_h, buf0_l, buf1_h, buf1_l, buf_lh, buf_hl, buf_ll, tmp;
//      var int buf0, buf1, buf2, buf3;

//      var int mulres0_h, mulres0_l;
//      var int mulres1_h, mulres1_l;
//      var int mulres2_h, mulres2_l;
//      var int acc_h, acc_l;
//      var int dotres_h, dotres_l;

        //set drawBuf to start of 2d draw buffer
        let drawBuf = ADDR_P2D;

        let room_mdl_addr = ROOM_MDLS(room_addr);
        let room_mdls_end = room_mdl_addr + ROOM_MDL_COUNT(room_addr);

        //loop over the room model array
        //find the transformed points (by the view matrix)
        //then clip and project them correctly
        while(room_mdl_addr < room_mdls_end) {
            //start of model's transformed 3d points
            let mdl_addr = PEEK(room_mdl_addr);
            let points_addr = MDL_P3D(mdl_addr);

            //reset the cull buffer
            let face_edge_map_addr = ADDR_EF;
            while(face_edge_map_addr < ADDR_EF_END) {
                do POKE(face_edge_map_addr, 0);
                let face_edge_map_addr = face_edge_map_addr + 1;
            }

            //determine which faces (and their respective edges) are to be culled
            let face_addr = ADDR_F;
            while(face_addr < ADDR_F_END) {
                //get an edge
                let edge_addr = PEEK(face_addr)*2 + ADDR_E;
                //get (the address of) 2 points on the face
                let src = ( 3*PEEK(edge_addr) ) + points_addr;
                let dst = ( 3*PEEK(edge_addr + 1) ) + points_addr;

                //pick any point on the face for the looking vector (just the position in view space)
                let x1 =(PEEK(src) + PEEK(dst)) / 2;
                let y1 =(PEEK(src + 1) + PEEK(dst + 1)) / 2;
                let z1 =(PEEK(src + 2) + PEEK(dst + 2)) / 2;

//              let edge_addr = PEEK(face_addr + 2) + ADDR_E;
//              let src = ( 3*PEEK(edge_addr) ) + points_addr;
//              let dst = ( 3*PEEK(edge_addr + 1) ) + points_addr;

//              let x1 = (x1 + ((PEEK(src) + PEEK(dst)) / 2)) / 2;
//              let y1 = (y1 + ((PEEK(src + 1) + PEEK(dst + 1))) / 2) / 2;
//              let z1 = (z1 + ((PEEK(src + 2) + PEEK(dst + 2))) / 2) / 2;

                //get normal vector
                let x_normal = PEEK(face_addr + 4);
                let y_normal = PEEK(face_addr + 5);
                let z_normal = PEEK(face_addr + 6);

                //transform normal vector
                VIEW_TRANSFORM(x_normal, y_normal, z_normal, x2, y2, z2);

                let x2 = x2 / 4;
                let y2 = y2 / 4;
                let z2 = z2 / 4;

                let x1 = x1 / 32;
                let y1 = y1 / 32;
                let z1 = z1 / 32;

                //do 32bit prec math
//              MULTIPLY_FULLPRECISION(x1, x2, mulres0_h, mulres0_l)
//              MULTIPLY_FULLPRECISION(y1, y2, mulres1_h, mulres1_l)
//              MULTIPLY_FULLPRECISION(z1, z2, mulres2_h, mulres2_l)

//              ADD_FULLPRECISION(mulres0_l, mulres0_h, mulres1_l, mulres1_h, acc_h, acc_l)
//              ADD_FULLPRECISION(mulres2_l, mulres2_h, acc_l, acc_h, dotres_h, dotres_l)

                //implicitly invert the looking vector
//              if( (dotres_h > 0) | ((dotres_h = 0)) )  {
                if( (x1*x2) + (y1*y2) > -(z1*z2) ) {
                    //culled
                    //do nothing
                } else {
                    do POKE(ADDR_EF + PEEK(face_addr), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+1), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+2), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+3), -1);
                    //not culled => set entries to true
                }

                let face_addr = face_addr + 7;
            }

            //fill the draw buffer with edges (pairs of 2d points)
            let edge_addr = ADDR_E;
            let face_edge_map_addr = ADDR_EF;
            while(edge_addr < ADDR_E_END) {

                if(PEEK(face_edge_map_addr)) {
                    //draw
                    //get the addresses of points of the edge at edge_addr
                    let src = ( 3*PEEK(edge_addr) ) + points_addr;
                    let dst = ( 3*PEEK(edge_addr + 1) ) + points_addr;

                    CLIP_PROJECT_BUFFER_EDGE(PEEK(src), PEEK(src+1), PEEK(src+2), PEEK(dst), PEEK(dst+1), PEEK(dst+2))
                } else {
                    //ignore culled
                }

                let edge_addr = edge_addr + 2;
                let face_edge_map_addr = face_edge_map_addr + 1;
            }

            let room_mdl_addr = room_mdl_addr + 1;
        }
        return;
    }

    #define CALCULATE_INTERSECTION_TERMS(X1, Y1, X2, Y2, XDIFF_OUT, YDIFF_OUT) \
        let XDIFF_OUT = X1 - X2; \\
        let YDIFF_OUT = Y1 - Y2;

//  #ifdef HIGH_PRECISION_ARITHMETIC
//  #define ORIENTATION_WRT_REF_EDGE(X, Y, X1_REF, Y1_REF, X2_REF, Y2_REF) \
//      (MULTIPLY_7_7_O14_INLINE((X2_REF - X1_REF), (Y - Y1_REF)) - MULTIPLY_7_7_O14_INLINE((Y2_REF - Y1_REF), (X - X1_REF)))

//  #define INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, X1, Y1, X2, Y2, XDIFF, YDIFF, REF_X1, REF_Y1, REF_X2, REF_Y2, REF_XDIFF, REF_YDIFF) \
//      let denom = (XDIFF*REF_YDIFF) - (YDIFF*REF_XDIFF); \\
//      let numer = ((X1 - REF_X1) * REF_YDIFF) - ((Y1 - REF_Y1) * REF_XDIFF); \\
// \\
//      if(denom) { \\
//          DIVIDE_7(numer, denom, buf1) \\
//          MULTIPLY_7_7_O7((-XDIFF), buf1, buf2); \\
//          MULTIPLY_7_7_O7((-YDIFF), buf1, buf3); \\
//          let X_OUT = X1 + buf2; \\
//          let Y_OUT = Y1 + buf3; \\
//      }
//  #endif

//  #ifndef HIGH_PRECISION_ARITHMETIC
    #define ORIENTATION_WRT_REF_EDGE(X, Y, X1_REF, Y1_REF, X2_REF, Y2_REF) \
        (((X2_REF - X1_REF) * (Y - Y1_REF)) - ((Y2_REF - Y1_REF) * (X - X1_REF)))

    #define INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, X1, Y1, X2, Y2, XDIFF, YDIFF, REF_X1, REF_Y1, REF_X2, REF_Y2, REF_XDIFF, REF_YDIFF) \
        let denom = ((XDIFF*REF_YDIFF)/32) - ((YDIFF*REF_XDIFF)/32); \\
        let numer = (((X1 - REF_X1) * REF_YDIFF)/32) - (((Y1 - REF_Y1) * REF_XDIFF)/32); \\
        if(denom) { \\
            let X_OUT = X1 + (((-XDIFF) * numer) / denom); \\
            let Y_OUT = Y1 + (((-YDIFF) * numer) / denom); \\
        }
//  #endif

    //assumes existence of the top, bottom, left, right vars
    #define SET_POLY_OUTBITS(X, Y, OUTBITS) \
        if(ORIENTATION_WRT_REF_EDGE(X, Y, V4(right, 0), V4(right, 1), V4(right, 2), V4(right, 3)) < 0) {\\
            let OUTBITS = OUTBITS | 2;\\
\\
        } else {\\
            if(ORIENTATION_WRT_REF_EDGE(X, Y, V4(left, 0), V4(left, 1), V4(left, 2), V4(left, 3)) < 0) {\\
                let OUTBITS =  OUTBITS | 1;\\
            }\\
        }\\
        if(ORIENTATION_WRT_REF_EDGE(X, Y, V4(top, 0), V4(top, 1), V4(top, 2), V4(top, 3)) < 0) {\\
            let OUTBITS = OUTBITS | 8;\\
\\
        } else {\\
            if(ORIENTATION_WRT_REF_EDGE(X, Y, V4(bottom, 0), V4(bottom, 1), V4(bottom, 2), V4(bottom, 3)) < 0) { \\
                let OUTBITS = OUTBITS | 4;\\
            }\\
        }

    #define CALCULATE_INTERSECTIONS(X_OUT, Y_OUT, OUTBITS) \
        if(OUTBITS) {\\
            if(OUTBITS & 3) {\\
                if(OUTBITS & 1) {\\
                    INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, x1_draw, y1_draw, x2_draw, y2_draw, edge_xdiff, edge_ydiff, \
                        V4(left, 0), V4(left, 1), V4(left, 2), V4(left, 3), left_xdiff, left_ydiff)\\
                } else {\\
                    INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, x1_draw, y1_draw, x2_draw, y2_draw, edge_xdiff, edge_ydiff, \
                        V4(right, 0), V4(right, 1), V4(right, 2), V4(right, 3), right_xdiff, right_ydiff)\\
                }\\
            } else {\\
                if(OUTBITS & 4) {\\
                    INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, x1_draw, y1_draw, x2_draw, y2_draw, edge_xdiff, edge_ydiff, \
                        V4(bottom, 0), V4(bottom, 1), V4(bottom, 2), V4(bottom, 3), bottom_xdiff, bottom_ydiff)\\
                } else {\\
                    INTERSECT_WRT_REF_EDGE(X_OUT, Y_OUT, x1_draw, y1_draw, x2_draw, y2_draw, edge_xdiff, edge_ydiff, \
                        V4(top, 0), V4(top, 1), V4(top, 2), V4(top, 3), top_xdiff, top_ydiff)\\
                }\\
            }\\
        }

    // prt: v3 topright, v3 bottomleft, v3 normal, int other_room_addr
    function void bufferEdgesInRoomPortaled(int prt_addr) {
        var int room_addr, room_mdl_addr, room_mdls_end;
        var int mdl_addr, points_addr, prt_points_addr, edge_addr, face_addr, face_edge_map_addr;

        var int x1_draw, y1_draw, x2_draw, y2_draw;
        var int x1_draw_tmp, y1_draw_tmp, x2_draw_tmp, y2_draw_tmp;
        var int x_normal, y_normal, z_normal;
        var int remainder, divbuf, denominator;

        //for finding intersections =================================================
        var int top_xdiff,    top_ydiff;
        var int bottom_xdiff, bottom_ydiff;
        var int left_xdiff,   left_ydiff;
        var int right_xdiff,  right_ydiff;
        var int edge_xdiff,   edge_ydiff;
        var int denom, numer, buf0, buf1, buf2, buf3;
        var int outbits_1, outbits_2;
        var int k;
        //===========================================================================

        //corners of the portal
        DECL_V3(tr, var);
        DECL_V3(br, var);
        DECL_V3(bl, var);
        DECL_V3(tl, var);

        //edges of the portal in screenspace
        DECL_V4(top, var);
        DECL_V4(bottom, var);
        DECL_V4(left, var);
        DECL_V4(right, var);

        //edges of the portal in screenspace in full precision
        DECL_V4(f_top, var);
        DECL_V4(f_bottom, var);
        DECL_V4(f_left, var);
        DECL_V4(f_right, var);

        //edge addresses
        var int src, dst;

        //stores draw edge flag
        var bool res;
        var bool winding;

        var int prt_visible_edges;
        let prt_visible_edges = 0;

        do Main.breakpoint();
        //draw nothing if portal faces away =========================================
        let prt_points_addr = PRT_P3D(prt_addr);

        FILL_V3_ADDR(tr, prt_points_addr);
        FILL_V3_ADDR(bl, prt_points_addr + 6);

        let x1 =(V3(tr, 0) + V3(bl, 0)) / 2;
        let y1 =(V3(tr, 1) + V3(bl, 1)) / 2;
        let z1 =(V3(tr, 2) + V3(bl, 2)) / 2;

        let x_normal = PEEK(prt_addr + 6);
        let y_normal = PEEK(prt_addr + 7);
        let z_normal = PEEK(prt_addr + 8);
        VIEW_TRANSFORM(x_normal, y_normal, z_normal, x2, y2, z2);

        let x1 = x1 / 32;
        let y1 = y1 / 32;
        let z1 = z1 / 32;

        let x2 = x2 / 4;
        let y2 = y2 / 4;
        let z2 = z2 / 4;

        if( (x1*x2) + (y1*y2) > -(z1*z2) ) {
            return;
        }

        do Main.breakpoint();
        //else draw portal boundary =================================================
        FILL_V3_ADDR(br, prt_points_addr + 3);
        FILL_V3_ADDR(tl, prt_points_addr + 9);

        //ensure they're counterclockwise for the correct occlusion
        //division by 4 against overflow issues
        if(~(V3(tr, 0) < V3(bl, 0)) ) {
            let winding = true;
            CLIP_PROJECT_EDGE(V3(tr, 0), V3(tr, 1), V3(tr, 2), V3(tl, 0), V3(tl, 1), V3(tl, 2))
            FILL_V4(f_top, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(top, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(tl, 0), V3(tl, 1), V3(tl, 2), V3(bl, 0), V3(bl, 1), V3(bl, 2))
            FILL_V4(f_left, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(left, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(bl, 0), V3(bl, 1), V3(bl, 2), V3(br, 0), V3(br, 1), V3(br, 2))
            FILL_V4(f_bottom, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(bottom, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(br, 0), V3(br, 1), V3(br, 2), V3(tr, 0), V3(tr, 1), V3(tr, 2))
            FILL_V4(f_right, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(right, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

        } else {
            let winding = false;
            CLIP_PROJECT_EDGE(V3(tl, 0), V3(tl, 1), V3(tl, 2), V3(tr, 0), V3(tr, 1), V3(tr, 2))
            FILL_V4(f_top, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(top, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(tr, 0), V3(tr, 1), V3(tr, 2), V3(br, 0), V3(br, 1), V3(br, 2))
            FILL_V4(f_right, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(right, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(br, 0), V3(br, 1), V3(br, 2), V3(bl, 0), V3(bl, 1), V3(bl, 2))
            FILL_V4(f_bottom, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(bottom, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);

            CLIP_PROJECT_EDGE(V3(bl, 0), V3(bl, 1), V3(bl, 2), V3(tl, 0), V3(tl, 1), V3(tl, 2))
            FILL_V4(f_left, x1_draw, y1_draw, x2_draw, y2_draw);
            FILL_V4(left, x1_draw / 2, y1_draw / 2, x2_draw / 2, y2_draw / 2);
            let prt_visible_edges = prt_visible_edges + (res & 1);
        }

        //discard when portal not in view
        if(prt_visible_edges = 0) {
            return;
        }

        CALCULATE_INTERSECTION_TERMS(V4(right, 0), V4(right, 1), V4(right, 2), V4(right, 3), right_xdiff, right_ydiff)
        CALCULATE_INTERSECTION_TERMS(V4(top, 0), V4(top, 1), V4(top, 2), V4(top, 3), top_xdiff, top_ydiff)
        CALCULATE_INTERSECTION_TERMS(V4(bottom, 0), V4(bottom, 1), V4(bottom, 2), V4(bottom, 3), bottom_xdiff, bottom_ydiff)
        CALCULATE_INTERSECTION_TERMS(V4(left, 0), V4(left, 1), V4(left, 2), V4(left, 3), left_xdiff, left_ydiff)

        //===========================================================================

        let room_addr = PRT_ROOM(prt_addr);
        let room_mdl_addr = ROOM_MDLS(room_addr);
        let room_mdls_end = room_mdl_addr + ROOM_MDL_COUNT(room_addr);

        //loop over the room model array
        //find the transformed points (by the view matrix)
        //then clip and project them correctly
        while(room_mdl_addr < room_mdls_end) {
            //start of model's transformed 3d points
            let mdl_addr = PEEK(room_mdl_addr);
            let points_addr = MDL_P3D(mdl_addr);

            //reset the cull buffer
            let face_edge_map_addr = ADDR_EF;
            while(face_edge_map_addr < ADDR_EF_END) {
                do POKE(face_edge_map_addr, 0);
                let face_edge_map_addr = face_edge_map_addr + 1;
            }

            //determine which faces (and their respective edges) are to be culled
            let face_addr = ADDR_F;
            while(face_addr < ADDR_F_END) {
                //get an edge
                let edge_addr = PEEK(face_addr)*2 + ADDR_E;
                //get (the address of) 2 points on the face
                let src = ( 3*PEEK(edge_addr) ) + points_addr;
                let dst = ( 3*PEEK(edge_addr + 1) ) + points_addr;

                //pick any point on the face for the looking vector (just the position in view space)
                let x1 =(PEEK(src) + PEEK(dst)) / 2;
                let y1 =(PEEK(src + 1) + PEEK(dst + 1)) / 2;
                let z1 =(PEEK(src + 2) + PEEK(dst + 2)) / 2;

                //get normal vector
                let x_normal = PEEK(face_addr + 4);
                let y_normal = PEEK(face_addr + 5);
                let z_normal = PEEK(face_addr + 6);

                //transform normal vector
                VIEW_TRANSFORM(x_normal, y_normal, z_normal, x2, y2, z2);

                let x2 = x2 / 4;
                let y2 = y2 / 4;
                let z2 = z2 / 4;

                let x1 = x1 / 32;
                let y1 = y1 / 32;
                let z1 = z1 / 32;

                //implicitly invert the looking vector
                if( (x1*x2) + (y1*y2) > -(z1*z2) ) {
                    //culled
                    //do nothing
                } else {
                    do POKE(ADDR_EF + PEEK(face_addr), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+1), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+2), -1);
                    do POKE(ADDR_EF + PEEK(face_addr+3), -1);
                    //not culled => set entries to true
                }

                let face_addr = face_addr + 7;
            }

            //fill the draw buffer with edges (pairs of 2d points)
            let edge_addr = ADDR_E;
            let face_edge_map_addr = ADDR_EF;
            while(edge_addr < ADDR_E_END) {

                if(PEEK(face_edge_map_addr)) {
                    //draw
                    //get the addresses of points of the edge at edge_addr
                    let src = ( 3*PEEK(edge_addr) ) + points_addr;
                    let dst = ( 3*PEEK(edge_addr + 1) ) + points_addr;

                    let x1 = PEEK(src); 
                    let y1 = PEEK(src+1); 
                    let z1 = PEEK(src+2); 
                
                    let x2 = PEEK(dst);
                    let y2 = PEEK(dst+1);
                    let z2 = PEEK(dst+2);
            
                    let res = Main.line();
                    if(res) {
                        PROJECT_EDGE(x1, y1, z1, x2, y2, z2); // outputs to .._draw

                        let x1_draw = x1_draw / 2;
                        let y1_draw = y1_draw / 2;
                        let x2_draw = x2_draw / 2;
                        let y2_draw = y2_draw / 2;

                        let x1_draw_tmp = x1_draw;
                        let y1_draw_tmp = y1_draw;
                        let x2_draw_tmp = x2_draw;
                        let y2_draw_tmp = y2_draw;

                        let k = 1;
                        while(k > 0) {
                            //precalculate some factors
                            CALCULATE_INTERSECTION_TERMS(x1_draw, y1_draw, x2_draw, y2_draw, edge_xdiff, edge_ydiff)

                            let outbits_1 = 0;
                            let outbits_2 = 0;
                            SET_POLY_OUTBITS(x1_draw, y1_draw, outbits_1)
                            SET_POLY_OUTBITS(x2_draw, y2_draw, outbits_2)

                            if(outbits_1 | outbits_2) {
                                if(outbits_1 & outbits_2) {
                                    let res = false;
                                } else {
                                    CALCULATE_INTERSECTIONS(x1_draw_tmp, y1_draw_tmp, outbits_1)
                                    CALCULATE_INTERSECTIONS(x2_draw_tmp, y2_draw_tmp, outbits_2)
                                }
                            }

                            let x1_draw = x1_draw_tmp;
                            let y1_draw = y1_draw_tmp;
                            let x2_draw = x2_draw_tmp;
                            let y2_draw = y2_draw_tmp;

                            let k = k - 1;
                        }
                    }

                    if(res) {
                        let x1_draw = x1_draw * 2 + 256;
                        let y1_draw = y1_draw * 2 + 128;
                        let x2_draw = x2_draw * 2 + 256;
                        let y2_draw = y2_draw * 2 + 128;

                        CLAMP(x1_draw, 1, 511);
                        CLAMP(y1_draw, 1, 255);
                        CLAMP(x2_draw, 1, 511);
                        CLAMP(y2_draw, 1, 255);

                        FILL_INARRAY_V4(drawBuf, x1_draw, y1_draw, x2_draw, y2_draw);
                        let drawBuf = drawBuf + 4;
                    }
                }


                let edge_addr = edge_addr + 2;
                let face_edge_map_addr = face_edge_map_addr + 1;
            }

            let room_mdl_addr = room_mdl_addr + 1;
        }

        #ifdef DRAW_PORTAL_EDGES
        let V4(f_top, 0) = V4(f_top, 0) + 256;
        let V4(f_right, 0) = V4(f_right, 0) + 256;
        let V4(f_left, 0) = V4(f_left, 0) + 256;
        let V4(f_bottom, 0) = V4(f_bottom, 0) + 256;
        let V4(f_top, 2) = V4(f_top, 2) + 256;
        let V4(f_right, 2) = V4(f_right, 2) + 256;
        let V4(f_left, 2) = V4(f_left, 2) + 256;
        let V4(f_bottom, 2) = V4(f_bottom, 2) + 256;
        CLAMP(V4(f_top, 0)   , 1, 511);
        CLAMP(V4(f_right, 0) , 1, 511);
        CLAMP(V4(f_left, 0)  , 1, 511);
        CLAMP(V4(f_bottom, 0), 1, 511);
        CLAMP(V4(f_top, 2)   , 1, 511);
        CLAMP(V4(f_right, 2) , 1, 511);
        CLAMP(V4(f_left, 2)  , 1, 511);
        CLAMP(V4(f_bottom, 2), 1, 511);

        let V4(f_top, 1) = V4(f_top, 1) + 128;
        let V4(f_right, 1) = V4(f_right, 1) + 128;
        let V4(f_left, 1) = V4(f_left, 1) + 128;
        let V4(f_bottom, 1) = V4(f_bottom, 1) + 128;
        let V4(f_top, 3) = V4(f_top, 3) + 128;
        let V4(f_right, 3) = V4(f_right, 3) + 128;
        let V4(f_left, 3) = V4(f_left, 3) + 128;
        let V4(f_bottom, 3) = V4(f_bottom, 3) + 128;
        CLAMP(V4(f_top, 1)   , 1, 255);
        CLAMP(V4(f_right, 1) , 1, 255);
        CLAMP(V4(f_left, 1)  , 1, 255);
        CLAMP(V4(f_bottom, 1), 1, 255);
        CLAMP(V4(f_top, 3)   , 1, 255);
        CLAMP(V4(f_right, 3) , 1, 255);
        CLAMP(V4(f_left, 3)  , 1, 255);
        CLAMP(V4(f_bottom, 3), 1, 255);

        FILL_INARRAY_V4(drawBuf, V4(f_right, 0), V4(f_right, 1), V4(f_right, 2), V4(f_right, 3));
        let drawBuf = drawBuf + 4;
        FILL_INARRAY_V4(drawBuf, V4(f_top, 0), V4(f_top, 1), V4(f_top, 2), V4(f_top, 3));
        let drawBuf = drawBuf + 4;
        FILL_INARRAY_V4(drawBuf, V4(f_bottom, 0), V4(f_bottom, 1), V4(f_bottom, 2), V4(f_bottom, 3));
        let drawBuf = drawBuf + 4;
        FILL_INARRAY_V4(drawBuf, V4(f_left, 0), V4(f_left, 1), V4(f_left, 2), V4(f_left, 3));
        let drawBuf = drawBuf + 4;
        #endif

        return;                 
    }                           
    function void draw() {      
        var int i;
        var int x1_draw, y1_draw, x2_draw, y2_draw;

        let i = ADDR_P2D;
        while(i < drawBuf) {

            //drawBuf stores edges, so get both points
            let x1_draw = PEEK(i);
            let y1_draw = PEEK(i+1);

            let x2_draw = PEEK(i+2);
            let y2_draw = PEEK(i+3);

            do Screen.drawLine(x1_draw, y1_draw, x2_draw, y2_draw);

            let i = i + 4;
        }
        return;
    }

    //calculates intercepts with the view frustum side planes
    #define CALCULATE_INTERCEPTS(X_RES, Y_RES, Z_RES, X_EXT_LOCAL, Y_EXT_LOCAL, X1, Y1, Z1, X2, Y2, Z2, OUTBITS) \
        if(OUTBITS) {\\
            if(OUTBITS & 3) {\\
                if(OUTBITS & 1) {\\
                    LEFT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2);\\
                    let X_RES = (2 * Z_RES);\\
                } else {\\
                    RIGHT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2);\\
                    let X_RES = ((-2) * Z_RES);\\
                }\\
            } else {\\
                if(OUTBITS & 4) {\\
                    BOTTOM_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2);\\
                    let Y_RES = (Z_RES);\\
                } else {\\
                    TOP_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2);\\
                    let Y_RES = (-Z_RES);\\
                }\\
            }\\
        }



    #ifdef HIGH_PRECISION_ARITHMETIC
    #define TOP_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (Y2 - Y1) + (Z2 - Z1); \\
        let denom = buf0; \\
        let numer = Y1 + Z1; \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        DIVIDE_7(numer, denom, buf1) \\
        let buf1 = buf1 / 4;\\
        MULTIPLY_7_7_O7(((X2 - X1)), buf1, buf2); \\
        MULTIPLY_7_7_O7(((Z2 - Z1)), buf1, buf3); \\
        let X_RES = X1 - (4*buf2); \\
        let Z_RES = Z1 - (4*buf3)

    #define BOTTOM_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (Y2 - Y1) - (Z2 - Z1); \\
        let denom = buf0; \\
        let numer = (Y1 - Z1); \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        DIVIDE_7(numer, denom, buf1) \\
        let buf1 = buf1 / 4;\\
        MULTIPLY_7_7_O7((X2 - X1), buf1, buf2); \\
        MULTIPLY_7_7_O7((Z2 - Z1), buf1, buf3); \\
        let X_RES = X1 - (4*buf2); \\
        let Z_RES = Z1 - (4*buf3)

    #define RIGHT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (X2 - X1) + (2*(Z2 - Z1)); \\
        let denom = buf0; \\
        let numer = (X1 + (2*Z1)); \\
        \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        DIVIDE_7(numer, denom, buf1) \\
        let buf1 = buf1 / 4;\\
        MULTIPLY_7_7_O7((Z2 - Z1), buf1, buf2); \\
        MULTIPLY_7_7_O7((Y2 - Y1), buf1, buf3); \\
        let Z_RES = Z1 - (4*buf2); \\
        let Y_RES = Y1 - (4*buf3)

    #define LEFT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (X2 - X1) - (2*(Z2 - Z1)); \\
        let denom = buf0; \\
        let numer = (X1 - (2*Z1)); \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        DIVIDE_7(numer, denom, buf1) \\
        let buf1 = buf1 / 4;\\
        MULTIPLY_7_7_O7((Z2 - Z1), buf1, buf2); \\
        MULTIPLY_7_7_O7((Y2 - Y1), buf1, buf3); \\
        let Z_RES = Z1 - (4*buf2); \\
        let Y_RES = Y1 - (4*buf3)

//  #define NEARPLANE_INTERCEPT(X_RES, Y_RES, X1, Y1, Z1, X2, Y2, Z2) \
//      let denom = (Z2 - Z1)/4; \\
//      let numer = (NEAR_PLANE - Z1)/4; \\
//      if(denom = 0) { \\
//          if(Z2 > Z1) {\\
//              let denom = 1; \\
//          } else {\\
//              let denom = -1; \\
//          }\\
//      }\\
//      DIVIDE_7(numer, denom, buf1) \\
//      MULTIPLY_7_7_O7((Y2 - Y1), buf1, buf2); \\
//      MULTIPLY_7_7_O7((X2 - X1), buf1, buf3); \\
//      let Y_RES = Y1 + buf2; \\
//      let X_RES = X1 + buf3
    #endif

    #ifndef HIGH_PRECISION_ARITHMETIC
    #define TOP_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (Y2 - Y1) + (Z2 - Z1); \\
        let denom = buf0 / 32; \\
        let numer = (Y1 + Z1) / 16; \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        let X_RES = X1 - ((((X2 - X1) / 2) * numer) / denom); \\
        let Z_RES = Z1 - ((((Z2 - Z1) / 2) * numer) / denom)

    #define BOTTOM_INTERCEPT(X_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (Y2 - Y1) - (Z2 - Z1); \\
        let denom = buf0 / 32; \\
        let numer = (Y1 - Z1) / 16; \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        let X_RES = X1 - ((((X2 - X1) / 2) * numer) / denom); \\
        let Z_RES = Z1 - ((((Z2 - Z1) / 2) * numer) / denom)

    #define RIGHT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (X2 - X1) + (2*(Z2 - Z1)); \\
        let denom = buf0 / 32; \\
        let numer = (X1 + (2*Z1)) / 16; \\
        \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        let Z_RES = Z1 - ((((Z2 - Z1) / 2) * numer) / denom); \\
        let Y_RES = Y1 - ((((Y2 - Y1) / 2) * numer) / denom)

    #define LEFT_INTERCEPT(Y_RES, Z_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let buf0 = (X2 - X1) - (2*(Z2 - Z1)); \\
        let denom = buf0 / 32; \\
        let numer = (X1 - (2*Z1)) / 16; \\
        if(denom = 0) { \\
            if(buf0 > 0) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        let Z_RES = Z1 - ((((Z2 - Z1) / 2) * numer) / denom); \\
        let Y_RES = Y1 - ((((Y2 - Y1) / 2) * numer) / denom)
    #endif

    #define NEARPLANE_INTERCEPT(X_RES, Y_RES, X1, Y1, Z1, X2, Y2, Z2) \
        let denom = (Z2 - Z1) / 8; \\
        let numer = (NEAR_PLANE - Z1) / 8; \\
        if(denom = 0) { \\
            if(Z2 > Z1) {\\
                let denom = 1; \\
            } else {\\
                let denom = -1; \\
            }\\
        }\\
        let Y_RES = Y1 + ((((Y2 - Y1)) * numer) / denom); \\
        let X_RES = X1 + ((((X2 - X1)) * numer) / denom)
        
    #define SET_OUTBITS(X, Y, X_EXT_LOCAL, Y_EXT_LOCAL, OUTBITS) \
        if(X > X_EXT_LOCAL ) {\\
            let OUTBITS = OUTBITS | 2;\\
\\
        } else {\\
            if(X < -X_EXT_LOCAL) {\\
                let OUTBITS =  OUTBITS | 1;\\
            }\\
        }\\
        if(Y > Y_EXT_LOCAL ) {\\
            let OUTBITS = OUTBITS | 8;\\
\\
        } else {\\
            if(Y < -Y_EXT_LOCAL) { \\
                let OUTBITS = OUTBITS | 4;\\
            }\\
        }


    //works on the statics x1 y1 z1 ...
    //if neccessary change x1, y1,... to clip an edge and output true
    //output false if edge non-visible
    function bool line() {
        var int x1_ext_local, y1_ext_local, x2_ext_local, y2_ext_local;
        var int outbits_1, outbits_2;
        var int x1_res, y1_res, z1_res, x2_res, y2_res, z2_res;
        var int denom, numer, mul_out, k;
        var int buf0, buf1, buf2, buf3;
        var int remainder, divbuf, denominator;

        let outbits_1 = 0;
        let outbits_2 = 0;
        let k = 2;

        //centered versions of min/max
        //called EXT for extent, since the frustum is symmetric and min = -max

        //forward is -z, so z bigger -> point futher back
        if(z1 > NEAR_PLANE) {
            if(z2 > NEAR_PLANE) {
                //both points behind <<<<<<<<<<<<<<<<<<<<<<<<<<<<<
                return false;
            } else {
                //p1 back, p2 front <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
                //=> put p1 in front
                NEARPLANE_INTERCEPT(x1_res, y1_res, x1, y1, z1, x2, y2, z2);
                let z1 = NEAR_PLANE;
                let x1 = x1_res;
                let y1 = y1_res;
            }

        } else {
            if(z2 > NEAR_PLANE) {

                //p1 front, p2 back <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
                //=> put p2 in front
                NEARPLANE_INTERCEPT(x2_res, y2_res, x1, y1, z1, x2, y2, z2);
                let z2 = NEAR_PLANE;
                let x2 = x2_res;
                let y2 = y2_res;
            }
        }

        //both points in front <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        //loops twice
        while(k > 0) {
            let x1_ext_local = -z1*2;
            let y1_ext_local = -z1;

            let x2_ext_local = -z2*2;
            let y2_ext_local = -z2;

            let outbits_1 = 0;
            let outbits_2 = 0;

            SET_OUTBITS(x1, y1, x1_ext_local, y1_ext_local, outbits_1)
            SET_OUTBITS(x2, y2, x2_ext_local, y2_ext_local, outbits_2)

            let x1_res = x1;
            let y1_res = y1;
            let z1_res = z1;

            let x2_res = x2;
            let y2_res = y2;
            let z2_res = z2;

            //LOGIC =====================================
            if(outbits_1 | outbits_2) {
                if(outbits_1 & outbits_2) {
                    return false;
                }
                CALCULATE_INTERCEPTS(x2_res, y2_res, z2_res, x2_ext_local, y2_ext_local, x1, y1, z1, x2, y2, z2, outbits_2)
                CALCULATE_INTERCEPTS(x1_res, y1_res, z1_res, x1_ext_local, y1_ext_local, x1, y1, z1, x2, y2, z2, outbits_1)
            }

            let x1 = x1_res;
            let y1 = y1_res;
            let z1 = z1_res;

            let x2 = x2_res;
            let y2 = y2_res;
            let z2 = z2_res;

            let k = k - 1;
        }

        return true;
    }

    function void breakpoint() {
        return;
    }

    function void pollKeyboard() {
        var int key;
        let key = Keyboard.keyPressed();

        //W forward
        //S backward
        //A left
        //D right
        if(key & 64) {
            if(key & 16) {
                if (key = 87) {
                    let V3(velocity, 0) = V3(velocity, 0) - sinYaw;
                    let V3(velocity, 2) = V3(velocity, 2) + cosYaw;
                } else {
                    if (key = 83) {
                        let V3(velocity, 0) = V3(velocity, 0) + sinYaw;
                        let V3(velocity, 2) = V3(velocity, 2) - cosYaw;
                    }
                }
            } else {
                if (key = 65) {
                    let V3(velocity, 0) = V3(velocity, 0) - cosYaw;
                    let V3(velocity, 2) = V3(velocity, 2) - sinYaw;
                } else {
                    if (key = 68) {
                        let V3(velocity, 0) = V3(velocity, 0) + cosYaw;
                        let V3(velocity, 2) = V3(velocity, 2) + sinYaw;
                    }
                }
            }
        } else {

            //up/down arrows change pitch (turn up/down)
            //right/left arrows change yaw (turn left/right)
            //space up
            //',' down
            if(key & 2) {

                if (key = 130) {
                    let yaw = (yaw + 1) & 255;
                } else {
                    if (key = 131) {
                        let pitch = (pitch - 1) & 255;
                    }
                } 
            } else {

                if(key & 128) {
                    if (key = 132) {
                        let yaw = (yaw - 1) & 255;
                    } else {
                        if (key = 133) {
                            let pitch = (pitch + 1) & 255;
                        }
                    }
                } else {

                    if (key = 32) {
                        #ifdef FLIGHT
                        let V3(pos, 1) = V3(pos, 1) + 1;
                        #endif

                        #ifndef FLIGHT
                        if(V3(pos, 1) < (EYE_LEVEL + 1)) {
                            let V3(velocity, 1) = V3(velocity, 1) - 196;
                        }
                        #endif

                    } else {
                        #ifdef FLIGHT
                        if (key = 44) {
                            let V3(pos, 1) = V3(pos, 1) - 1;
                        }
                        #endif
                    }
                }

            }
        }

        #define POS_FRAC_MAGNITUDE 255

        #ifndef FLIGHT
        if(((V3(pos, 1) < EYE_LEVEL) | (V3(posFrac, 1) > -128 & V3(pos, 1) = EYE_LEVEL))) {
            let V3(posFrac, 1) = -128;
            let V3(pos, 1) = EYE_LEVEL;
            let V3(velocity, 1) = 0;
        } else {
            if((V3(velocity, 1) > 128) | ~(V3(pos, 1) > EYE_LEVEL)) {
                //nothing
            } else {
                let V3(velocity, 1) = V3(velocity, 1) + 24;
            }
        }
        #endif

        let V3(posFrac, 0) = V3(posFrac, 0) + (V3(velocity, 0) / 4);
        let V3(velocity, 0) = V3(velocity, 0) / 4;

        let V3(posFrac, 1) = V3(posFrac, 1) + V3(velocity, 1);

        let V3(posFrac, 2) = V3(posFrac, 2) + (V3(velocity, 2) / 4);
        let V3(velocity, 2) = V3(velocity, 2) / 4;

        if(V3(posFrac, 0) < -POS_FRAC_MAGNITUDE) {
            let V3(posFrac, 0) = V3(posFrac, 0) + POS_FRAC_MAGNITUDE;
            let V3(pos, 0) = V3(pos, 0) + 1;
        } else {
            if(V3(posFrac, 0) > 1) {
                let V3(posFrac, 0) = V3(posFrac, 0) - POS_FRAC_MAGNITUDE;
                let V3(pos, 0) = V3(pos, 0) - 1;
            } 
        }

        if(V3(posFrac, 1) < -POS_FRAC_MAGNITUDE) {
            let V3(posFrac, 1) = V3(posFrac, 1) + POS_FRAC_MAGNITUDE;
            let V3(pos, 1) = V3(pos, 1) + 1;
        } else {
            if(V3(posFrac, 1) > 1) {
                let V3(posFrac, 1) = V3(posFrac, 1) - POS_FRAC_MAGNITUDE;
                let V3(pos, 1) = V3(pos, 1) - 1;
            } 
        }

        if(V3(posFrac, 2) < -POS_FRAC_MAGNITUDE) {
            let V3(posFrac, 2) = V3(posFrac, 2) + POS_FRAC_MAGNITUDE;
            let V3(pos, 2) = V3(pos, 2) + 1;
        } else {
            if(V3(posFrac, 2) > -1) {
                let V3(posFrac, 2) = V3(posFrac, 2) - POS_FRAC_MAGNITUDE;
                let V3(pos, 2) = V3(pos, 2) - 1;
            } 
        }

        #ifdef FLIGHT
            let V3(velocity, 0) = 0;
            let V3(velocity, 1) = 0;
            let V3(velocity, 2) = 0;
        #endif

        return;
    }

    function void calcViewMat() {
        var int sinPitch, cosPitch;

        //SCALING signed 2^7
        let sinYaw = Main.sin(yaw);
        let cosYaw = Main.cos(yaw);
        let sinPitch = Main.sin(pitch);
        let cosPitch = Main.cos(pitch);

        //set scaling to signed 2^7
        let M3X3(view, 0, 0) = cosYaw;
//      let M3X3(view, 0, 1) = 0; // <- always
        let M3X3(view, 0, 2) = sinYaw;

        let M3X3(view, 1, 0) = (sinYaw * sinPitch) / 128;
        let M3X3(view, 1, 1) = cosPitch;
        let M3X3(view, 1, 2) = -(cosYaw * sinPitch) / 128;

        let M3X3(view, 2, 0) = -(sinYaw * cosPitch) / 128;
        let M3X3(view, 2, 1) = sinPitch; 
        let M3X3(view, 2, 2) = (cosYaw * cosPitch) / 128;

        return;
    }

    function int cos(int angle) {
    let angle = (angle + 64) & 255;
        if(angle < 128) {
            return -sinTable[angle];
        } else {
            return sinTable[angle - 128];
        }
    }

    function int sin(int angle) {
        if(angle < 128) {
            return -sinTable[angle];
        } else {
            return sinTable[angle - 128];
        }
    }

    function void fillPoints() {
        var int i, j, lim;
        var int sidesize, p3d_addr, p3d_r_addr, mdl_addr, room_addr;
        var int room1, room2, room3;
        var int cube3dOffset;

        //save model prototype 
        let p3d_addr = ADDR_P3D_CUBE;
        let p3d_addr = Cube.vertices(p3d_addr);
        do Cube.faces(ADDR_F);
        do Cube.edges(ADDR_E);


        //create model instances and fill rooms
        let mdl_addr = ADDR_MDL;
        let p3d_r_addr = ADDR_P3D_R;
        let cube3dOffset = (CUBE_VERTEX_COUNT*3);

        do Rooms.init(ADDR_ROOM, MODEL_SIZE);

        //ROOM 1 ===========================================================
        // takes bounding box:
        // int x_min, int x_max, int y_min, int y_max, int z_min, int z_max
        let room1 = Rooms.newRoom(-32,32,   -32,32,   -1,7);

        //Cube.model returns addr of next available slot, so we have to assign the mdl_addr
        //beforehand
        do Rooms.addModel(mdl_addr);

        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  -6,0,-1,  2,8,8);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);

        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  6,0,-1,  2, 8, 8);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;
        do Rooms.finalizeModels();

        // v3 topright, v3 bottomleft, v3 normal, otherRoomAddr set to 0 as it's not known yet
        let p3d_r_addr = Rooms.addPortal(2,8,-1,   0,0,-1,   0,0, 1,   0,  p3d_r_addr);
        let p3d_r_addr = Rooms.addPortal(2,8, 7,   0,0, 7,  0,0,-1,   0,  p3d_r_addr);

        do Rooms.finalizePortals();

        //ROOM 2 ===========================================================
        let room2 = Rooms.newRoom(-32,32,  -32,32,  8,20);

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  -4,1,12,  1,1,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  -3,0,12,  4,1,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  1,1,12,  1,1,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  -3,3,12,  1,2,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  0,3,12,  1,2,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;

        do Rooms.addModel(mdl_addr);
        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  -4,0,10,  1,1,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;
        do Rooms.finalizeModels();

        let p3d_r_addr = Rooms.addPortal(5,8, 7,   -5,0, 7,   0,0, 1,   room1,  p3d_r_addr);
        do Rooms.finalizePortals();

        //ROOM 3 ===========================================================
        let room3 = Rooms.newRoom(-32,32,  -32,32,  -20,-2);

        do Rooms.addModel(mdl_addr);

        let mdl_addr = Cube.model(mdl_addr, p3d_r_addr,  0,0,-4,  1,1,1);
        let p3d_r_addr = p3d_r_addr + cube3dOffset;
        do Rooms.finalizeModels();

        let p3d_r_addr = Rooms.addPortal(5,8,-1,   -5,0,-1,   0,0, -1,   room1,  p3d_r_addr);
        let room_addr_end = Rooms.finalizePortals();

        //ROOM 1 PORTALS ===================================================
        do Rooms.setPortalOf(room1, 0, room3);
        do Rooms.setPortalOf(room1, 1, room2);

        return;
    }

    function void fillSinTable() {
        let sinTable = Array.new(128);

        let sinTable[0] = 0; let sinTable[1] = 3; let sinTable[2] = 6; let sinTable[3] = 9;
        let sinTable[4] = 13; let sinTable[5] = 16; let sinTable[6] = 19; let sinTable[7] = 22;
        let sinTable[8] = 25; let sinTable[9] = 28; let sinTable[10] = 31; let sinTable[11] = 34;
        let sinTable[12] = 37; let sinTable[13] = 40; let sinTable[14] = 43; let sinTable[15] = 46;
        let sinTable[16] = 49; let sinTable[17] = 52; let sinTable[18] = 55; let sinTable[19] = 58;
        let sinTable[20] = 60; let sinTable[21] = 63; let sinTable[22] = 66; let sinTable[23] = 68;
        let sinTable[24] = 71; let sinTable[25] = 74; let sinTable[26] = 76; let sinTable[27] = 79;
        let sinTable[28] = 81; let sinTable[29] = 84; let sinTable[30] = 86; let sinTable[31] = 88;
        let sinTable[32] = 91; let sinTable[33] = 93; let sinTable[34] = 95; let sinTable[35] = 97;
        let sinTable[36] = 99; let sinTable[37] = 101; let sinTable[38] = 103; let sinTable[39] = 105;
        let sinTable[40] = 106; let sinTable[41] = 108; let sinTable[42] = 110; let sinTable[43] = 111;
        let sinTable[44] = 113; let sinTable[45] = 114; let sinTable[46] = 116; let sinTable[47] = 117;
        let sinTable[48] = 118; let sinTable[49] = 119; let sinTable[50] = 121; let sinTable[51] = 122;
        let sinTable[52] = 122; let sinTable[53] = 123; let sinTable[54] = 124; let sinTable[55] = 125;
        let sinTable[56] = 126; let sinTable[57] = 126; let sinTable[58] = 127; let sinTable[59] = 127;
        let sinTable[60] = 127; let sinTable[61] = 128; let sinTable[62] = 128; let sinTable[63] = 128;
        let sinTable[64] = 128; let sinTable[65] = 128; let sinTable[66] = 128; let sinTable[67] = 128;
        let sinTable[68] = 127; let sinTable[69] = 127; let sinTable[70] = 127; let sinTable[71] = 126;
        let sinTable[72] = 126; let sinTable[73] = 125; let sinTable[74] = 124; let sinTable[75] = 123;
        let sinTable[76] = 122; let sinTable[77] = 122; let sinTable[78] = 121; let sinTable[79] = 119;
        let sinTable[80] = 118; let sinTable[81] = 117; let sinTable[82] = 116; let sinTable[83] = 114;
        let sinTable[84] = 113; let sinTable[85] = 111; let sinTable[86] = 110; let sinTable[87] = 108;
        let sinTable[88] = 106; let sinTable[89] = 105; let sinTable[90] = 103; let sinTable[91] = 101;
        let sinTable[92] = 99; let sinTable[93] = 97; let sinTable[94] = 95; let sinTable[95] = 93;
        let sinTable[96] = 91; let sinTable[97] = 88; let sinTable[98] = 86; let sinTable[99] = 84;
        let sinTable[100] = 81; let sinTable[101] = 79; let sinTable[102] = 76; let sinTable[103] = 74;
        let sinTable[104] = 71; let sinTable[105] = 68; let sinTable[106] = 66; let sinTable[107] = 63;
        let sinTable[108] = 60; let sinTable[109] = 58; let sinTable[110] = 55; let sinTable[111] = 52;
        let sinTable[112] = 49; let sinTable[113] = 46; let sinTable[114] = 43; let sinTable[115] = 40;
        let sinTable[116] = 37; let sinTable[117] = 34; let sinTable[118] = 31; let sinTable[119] = 28;
        let sinTable[120] = 25; let sinTable[121] = 22; let sinTable[122] = 19; let sinTable[123] = 16;
        let sinTable[124] = 13; let sinTable[125] = 9; let sinTable[126] = 6; let sinTable[127] = 3;
        return;
    }
}

