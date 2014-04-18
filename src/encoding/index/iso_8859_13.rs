// AUTOGENERATED FROM index-iso-8859-13.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-13.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 8221, 162, 163, 164, 8222, 166, 167, 216, 169, 342, 171,
    172, 173, 174, 198, 176, 177, 178, 179, 8220, 181, 182, 183, 248, 185, 343,
    187, 188, 189, 190, 230, 260, 302, 256, 262, 196, 197, 280, 274, 268, 201,
    377, 278, 290, 310, 298, 315, 352, 323, 325, 211, 332, 213, 214, 215, 370,
    321, 346, 362, 220, 379, 381, 223, 261, 303, 257, 263, 228, 229, 281, 275,
    269, 233, 378, 279, 291, 311, 299, 316, 353, 324, 326, 243, 333, 245, 246,
    247, 371, 322, 347, 363, 252, 380, 382, 8217,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 128, 129 => 129, 130 => 130, 131 => 131, 132 => 132, 133 => 133,
        134 => 134, 135 => 135, 136 => 136, 137 => 137, 138 => 138, 139 => 139,
        140 => 140, 141 => 141, 142 => 142, 143 => 143, 144 => 144, 145 => 145,
        146 => 146, 147 => 147, 148 => 148, 149 => 149, 150 => 150, 151 => 151,
        152 => 152, 153 => 153, 154 => 154, 155 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 8221 => 161, 162 => 162,
        163 => 163, 164 => 164, 8222 => 165, 166 => 166, 167 => 167,
        216 => 168, 169 => 169, 342 => 170, 171 => 171, 172 => 172, 173 => 173,
        174 => 174, 198 => 175, 176 => 176, 177 => 177, 178 => 178, 179 => 179,
        8220 => 180, 181 => 181, 182 => 182, 183 => 183, 248 => 184,
        185 => 185, 343 => 186, 187 => 187, 188 => 188, 189 => 189, 190 => 190,
        230 => 191, 260 => 192, 302 => 193, 256 => 194, 262 => 195, 196 => 196,
        197 => 197, 280 => 198, 274 => 199, 268 => 200, 201 => 201, 377 => 202,
        278 => 203, 290 => 204, 310 => 205, 298 => 206, 315 => 207, 352 => 208,
        323 => 209, 325 => 210, 211 => 211, 332 => 212, 213 => 213, 214 => 214,
        215 => 215, 370 => 216, 321 => 217, 346 => 218, 362 => 219, 220 => 220,
        379 => 221, 381 => 222, 223 => 223, 261 => 224, 303 => 225, 257 => 226,
        263 => 227, 228 => 228, 229 => 229, 281 => 230, 275 => 231, 269 => 232,
        233 => 233, 378 => 234, 279 => 235, 291 => 236, 311 => 237, 299 => 238,
        316 => 239, 353 => 240, 324 => 241, 326 => 242, 243 => 243, 333 => 244,
        245 => 245, 246 => 246, 247 => 247, 371 => 248, 322 => 249, 347 => 250,
        363 => 251, 252 => 252, 380 => 253, 382 => 254, 8217 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}