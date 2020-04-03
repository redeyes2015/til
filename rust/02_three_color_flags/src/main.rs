/*
// https://openhome.cc/Gossip/AlgorithmGossip/ThreeColorsFlags.htm
Procedure MOVE(Flags[])
    w = 0
    b = 0
    r = LENGTH(Flags) - 1

    WHILE(Flags[w] == 'B' && w < LENGTH(Flags))
        b = b + 1
        w = w + 1

    WHILE(Flags[r] == 'R' && r > 0)
        r = r - 1

    WHILE(w <= r)
        IF(Flags[w] == 'W')
            w = w + 1
        ELSE IF(Flags[w] == 'B')
            SWAP(Flags[], b, w)
            b = b + 1
            w = w + 1
        ELSE
            SWAP(Flags[], r, w)
            r = r - 1
*/

fn arrange (flags : & mut [u8]) {
    let mut b = 0;
    let mut w = 0;
    let mut r = flags.len() - 1;

    while flags[w] == b'B' && w < flags.len() {
        b += 1;
        w += 1;
    }

    while flags[r] == b'R' && r > 0 {
        r -= 1;
    }

    while w <= r {
        if flags[w] == b'W' {
            w += 1;
        } else if flags[w] == b'B' {
            flags.swap(b, w);
            b += 1;
            w += 1;
        } else {
            flags.swap(r, w);
            r -= 1;
        }
    }

}

fn main() {
    let mut flags = b"BRWBWR".to_vec();
    arrange(flags.as_mut_slice());
    println!("{}", String::from_utf8(flags).unwrap());
}
