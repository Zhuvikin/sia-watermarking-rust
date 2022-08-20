static TEST_MATRIX: &'static [[f64; 32]; 32] = &[
    [
        232., 31., 6., 112., 62., 181., 33., 54., 27., 91., 60., 154., 118., 54., 157., 58., 126.,
        224., 130., 23., 255., 9., 254., 12., 32., 10., 20., 176., 25., 95., 41., 194.,
    ],
    [
        29., 100., 176., 77., 2., 119., 208., 115., 119., 30., 153., 221., 161., 155., 99., 249.,
        81., 80., 114., 13., 16., 178., 240., 124., 128., 8., 254., 222., 4., 53., 143., 224.,
    ],
    [
        181., 136., 69., 77., 121., 97., 42., 211., 136., 35., 231., 84., 25., 146., 3., 52., 81.,
        23., 242., 64., 251., 182., 236., 132., 201., 81., 16., 202., 169., 71., 199., 135.,
    ],
    [
        152., 0., 128., 163., 148., 0., 75., 247., 186., 31., 57., 113., 198., 41., 175., 11.,
        134., 215., 121., 24., 166., 78., 93., 217., 86., 100., 99., 148., 144., 37., 39., 218.,
    ],
    [
        76., 202., 171., 224., 22., 133., 159., 40., 67., 233., 93., 134., 142., 36., 74., 71.,
        201., 247., 252., 60., 252., 28., 56., 99., 125., 188., 219., 110., 161., 141., 98., 70.,
    ],
    [
        3., 106., 58., 239., 131., 139., 150., 86., 194., 214., 83., 227., 251., 225., 248., 4.,
        88., 197., 115., 30., 91., 133., 105., 141., 91., 57., 155., 83., 121., 5., 152., 13.,
    ],
    [
        160., 212., 98., 33., 107., 37., 141., 187., 8., 63., 177., 92., 61., 204., 120., 84.,
        179., 247., 98., 174., 246., 120., 152., 140., 58., 61., 198., 103., 225., 250., 182.,
        252.,
    ],
    [
        95., 134., 163., 75., 63., 11., 135., 14., 64., 114., 9., 181., 90., 8., 247., 4., 48.,
        77., 220., 173., 173., 98., 236., 225., 24., 57., 249., 4., 83., 141., 162., 145.,
    ],
    [
        87., 28., 231., 95., 38., 8., 45., 56., 143., 156., 176., 131., 90., 77., 126., 112., 71.,
        53., 50., 107., 182., 121., 89., 165., 44., 17., 123., 146., 9., 193., 145., 169.,
    ],
    [
        180., 110., 70., 251., 238., 140., 224., 127., 55., 198., 82., 96., 27., 184., 33., 57.,
        233., 166., 46., 163., 162., 232., 37., 119., 10., 249., 57., 125., 211., 10., 108., 89.,
    ],
    [
        189., 18., 102., 249., 94., 94., 11., 178., 42., 216., 168., 59., 234., 237., 148., 76.,
        203., 181., 11., 198., 149., 126., 207., 6., 242., 247., 184., 148., 144., 132., 186.,
        147.,
    ],
    [
        96., 172., 152., 73., 51., 182., 19., 16., 236., 150., 4., 41., 29., 66., 21., 117., 248.,
        20., 90., 173., 6., 99., 168., 190., 18., 191., 171., 120., 119., 53., 236., 38.,
    ],
    [
        75., 174., 201., 96., 117., 204., 103., 59., 158., 68., 127., 7., 172., 165., 200., 185.,
        125., 223., 159., 118., 9., 18., 95., 150., 38., 204., 89., 170., 164., 103., 14., 152.,
    ],
    [
        128., 150., 182., 41., 174., 232., 21., 133., 19., 86., 35., 233., 187., 146., 220., 68.,
        235., 155., 19., 143., 232., 36., 126., 68., 172., 8., 160., 178., 91., 211., 254., 194.,
    ],
    [
        156., 191., 234., 230., 80., 103., 225., 54., 77., 140., 230., 11., 123., 249., 154., 18.,
        206., 170., 4., 235., 82., 157., 215., 77., 79., 244., 156., 31., 254., 177., 165., 178.,
    ],
    [
        150., 88., 4., 139., 15., 147., 247., 37., 68., 133., 108., 14., 168., 106., 37., 105.,
        18., 216., 190., 156., 158., 255., 107., 29., 207., 42., 246., 18., 115., 33., 230., 168.,
    ],
    [
        119., 22., 176., 107., 12., 198., 18., 218., 102., 139., 253., 176., 21., 151., 113., 159.,
        40., 147., 84., 228., 188., 244., 184., 155., 3., 158., 31., 227., 117., 31., 83., 1.,
    ],
    [
        21., 254., 137., 156., 111., 3., 240., 135., 37., 255., 244., 11., 22., 172., 254., 72.,
        219., 8., 176., 241., 173., 156., 198., 31., 26., 44., 157., 168., 169., 181., 219., 211.,
    ],
    [
        180., 57., 68., 88., 2., 154., 102., 74., 71., 128., 153., 175., 129., 245., 51., 15., 80.,
        40., 194., 138., 95., 228., 178., 166., 198., 217., 141., 232., 245., 232., 251., 156.,
    ],
    [
        3., 46., 59., 182., 78., 204., 59., 139., 93., 136., 110., 187., 59., 121., 77., 69., 173.,
        127., 113., 20., 3., 150., 144., 252., 20., 93., 53., 191., 89., 46., 200., 127.,
    ],
    [
        177., 182., 99., 224., 164., 2., 96., 127., 169., 56., 122., 123., 36., 53., 166., 220.,
        168., 51., 131., 250., 74., 122., 92., 41., 146., 14., 199., 93., 21., 41., 150., 41.,
    ],
    [
        31., 189., 220., 159., 104., 14., 154., 27., 152., 198., 185., 121., 134., 140., 199.,
        123., 253., 64., 120., 79., 89., 154., 203., 36., 250., 29., 79., 154., 197., 105., 187.,
        21.,
    ],
    [
        241., 164., 90., 90., 249., 207., 237., 74., 181., 199., 227., 226., 36., 66., 3., 209.,
        111., 56., 227., 149., 94., 75., 75., 102., 100., 214., 35., 204., 8., 204., 2., 134.,
    ],
    [
        218., 239., 69., 211., 252., 144., 153., 196., 92., 43., 162., 216., 213., 125., 245., 16.,
        238., 8., 49., 246., 229., 54., 53., 149., 54., 190., 155., 159., 51., 253., 47., 135.,
    ],
    [
        252., 54., 1., 231., 50., 195., 237., 129., 177., 14., 97., 179., 81., 104., 88., 93.,
        199., 64., 143., 12., 189., 183., 241., 8., 28., 131., 237., 189., 95., 75., 206., 219.,
    ],
    [
        129., 96., 69., 150., 232., 123., 199., 126., 142., 83., 116., 205., 219., 205., 101.,
        237., 141., 109., 179., 112., 46., 133., 70., 56., 236., 111., 130., 239., 204., 87., 208.,
        160.,
    ],
    [
        210., 247., 71., 153., 138., 155., 129., 185., 232., 192., 55., 18., 14., 76., 243., 71.,
        237., 84., 84., 237., 97., 187., 8., 117., 23., 1., 118., 149., 102., 44., 183., 211.,
    ],
    [
        29., 182., 31., 16., 87., 165., 46., 115., 191., 73., 145., 231., 81., 223., 184., 98.,
        29., 137., 136., 135., 170., 240., 165., 191., 133., 239., 182., 190., 49., 145., 29.,
        145.,
    ],
    [
        49., 32., 19., 5., 138., 146., 109., 91., 179., 131., 7., 47., 10., 30., 142., 124., 8.,
        29., 240., 249., 113., 134., 55., 222., 82., 254., 12., 73., 54., 139., 219., 101.,
    ],
    [
        254., 29., 104., 47., 127., 20., 197., 79., 113., 232., 207., 57., 58., 173., 159., 196.,
        100., 92., 90., 225., 7., 177., 230., 202., 68., 231., 72., 185., 163., 103., 78., 49.,
    ],
    [
        122., 201., 119., 177., 250., 201., 138., 176., 210., 131., 124., 9., 40., 167., 49., 168.,
        252., 109., 195., 127., 12., 30., 138., 104., 241., 26., 75., 192., 110., 1., 59., 226.,
    ],
    [
        167., 31., 75., 130., 27., 112., 160., 117., 11., 139., 153., 54., 73., 83., 106., 179.,
        201., 12., 53., 40., 39., 43., 202., 60., 26., 61., 202., 113., 189., 23., 160., 105.,
    ],
];

pub fn get_test_32_32_matrix() -> Vec<Vec<f64>> {
    let mut result = vec![];
    for row in TEST_MATRIX.iter() {
        let mut vec = vec![0.; row.len()];
        vec.copy_from_slice(row);
        result.push(vec)
    }
    result
}
