// place files you want to import through the `$lib` alias in this folder.
export type LedgerEntry = {
    _id: {
        $oid: string;
    };
    arrival: {
        $date: {
            $numberLong: string;
        };
    };
    departure: {
        $date: {
            $numberLong: string;
        };
    };
    posted_on: {
        $date: {
            $numberLong: string;
        };
    };
    creator_id: string;
    num_staying: number;
    profile_pic: string;
    name: string;
};