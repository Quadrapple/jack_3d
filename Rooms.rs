#define FILL_INARRAY_V2(ADDR, V_0, V_1) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1)

#define FILL_INARRAY_V3(ADDR, V_0, V_1, V_2) \
    do POKE(ADDR, V_0); \\
    do POKE(ADDR+1, V_1); \\
    do POKE(ADDR+2, V_2)

#define PEEK(ADDR) \
    Memory.peek(ADDR)

#define POKE(ADDR, VAL) \
    Memory.poke(ADDR, VAL)

//address of the room array
//room <- size 8 + model_amount + PORTAL_STRIDE*portal_amount
// int x_min, x_max, y_min, y_max, z_min, z_max <- room bounding box
// int model_amount
// int mdl1, mdl2, mdl3,... <- array of addresses of models
// 
// int portal_amount
// portal prt1, prt2,... <- a portal is stored directly as a sequence of 11 ints, not as an address
// prt: v3 topright, v3 bottomleft, v3 normal, int other_room_addr, int output3d <- size 11

#define PORTAL_STRIDE 11

class Rooms {
    static int mdl_stride;
    static int rooms_addr_begin;

    //current available room
    static int room_addr;

    static int mdl_count;
    static int mdl_local; // address of the model entry

    static int prt_count;
    static int prt_local; // address of the portal entry

    function void init(int roomsStartAddr, int modelSize) {
        let rooms_addr_begin = roomsStartAddr;
        let mdl_stride = modelSize;
        let room_addr = rooms_addr_begin;

        return;
    }

    function int newRoom(int x_min, int x_max, int y_min, int y_max, int z_min, int z_max) {
        var int room_start;
        let room_start = room_addr;

        //save bounding box
        FILL_INARRAY_V2(room_addr, x_min, x_max);
        FILL_INARRAY_V2(room_addr + 2, y_min, y_max);
        FILL_INARRAY_V2(room_addr + 4, z_min, z_max);

        //make sure room_addr points to mdl_count
        let room_addr = room_addr + 6;

        //make sure mdl_local points to first entry of model addr array
        let mdl_local = room_addr + 1;

        let mdl_count = 0;
        let prt_count = 0;
        //prt local unknown yet

        return room_start;
    }

    function void addModel(int mdl_addr) {
        do POKE(mdl_local, mdl_addr);
        let mdl_local = mdl_local + 1;
        let mdl_count = mdl_count + 1;

        return;
    }

    function void addModels(int mdl_addr_begin, int mdl_addr_end) {
        var int mdl_addr;
        let mdl_addr = mdl_addr_begin;
        while(mdl_addr < mdl_addr_end) {
            do Rooms.addModel(mdl_addr);
            let mdl_addr = mdl_addr + mdl_stride;
        }

        return;
    }

    function void finalizeModels() {
        //save the model count
        do POKE(room_addr, mdl_count);

        //advance address to point at prt_count
        let room_addr = room_addr + 1 + mdl_count;

        //point at first portal array entry
        let prt_local = room_addr + 1;
        return;
    }


    //ALL MODELS HAVE TO BE ADDED AND FINALIZED BEFORE PORTALS
    //assumes room_addr points to prt_count
    //assumes prt_local points to next available portal array entry slot
    function int addPortal(int x_tr, int y_tr, int z_tr, int x_bl, int y_bl, int z_bl, int x_normal, int y_normal, int z_normal, int otherRoomAddr,
        int outbuf3d) {
        FILL_INARRAY_V3(prt_local, x_tr, y_tr, z_tr);
        FILL_INARRAY_V3(prt_local + 3, x_bl, y_bl, z_tr);
        FILL_INARRAY_V3(prt_local + 6, x_normal, y_normal, z_normal);
        do POKE(prt_local + 9, otherRoomAddr);
        do POKE(prt_local + 10, outbuf3d);

        let prt_local = prt_local + PORTAL_STRIDE;
        let prt_count = prt_count + 1;

        //+12 for 4 points
        return outbuf3d + 12;
    }

    //NO ADDS TO ROOM AFTER THIS POINT
    function int finalizePortals() {
        //save the portal count
        do POKE(room_addr, prt_count);

        //advance available room address to end
        let room_addr = prt_local;
        return room_addr;
    }

    function void setPortalOf(int roomAddr, int portalInd, int otherRoomAddr) {
        var int room_current;
        var int mdl_count_current;
        var int prt_count_current;

        let room_current = roomAddr + 6;
        let mdl_count_current = PEEK(room_current);

        let room_current = room_current + 1 + mdl_count_current;
        let prt_count_current = PEEK(room_current);

        let room_current = room_current + 1 + (PORTAL_STRIDE * portalInd);

        if(portalInd < prt_count_current) {
            let room_current = room_current + 9;
            do POKE(room_current, otherRoomAddr);
        } else {
            do Sys.error(1001);
        }

        return;
    }
}
