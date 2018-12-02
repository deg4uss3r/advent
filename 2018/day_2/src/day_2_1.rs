use std::collections::HashMap;

fn parse_letters(input: &str) -> HashMap<&str, HashMap<char, u64>> {
    let words: Vec<&str> = input.split("\n").collect();
    let mut master_word_count: HashMap<&str, HashMap<char, u64>> = HashMap::new();

    for word in words.iter() {
        let mut letter_count: HashMap<char, u64> = HashMap::new();
        for c in word.chars() {
            let counter = letter_count.entry(c).or_insert(0);
                *counter += 1;
        }
        
        master_word_count.insert(word, letter_count);
    }

    master_word_count
}

fn calculate_checksum(parsed_letters: HashMap<&str, HashMap<char, u64>>) -> u64 {
    let mut two_count = 0;
    let mut three_count = 0;

    
    for (k,v) in parsed_letters.iter() {
        let mut add_two = false;
        let mut add_three = false;
        
        for (letter,count) in v.iter() {
            if count == &2 {
                add_two = true;
            }        
            
            if count == &3 {
                add_three = true;
            }
        }
        
        if add_two {
            two_count+=1;
        }
        
        if add_three {
            three_count+=1;
        }
    }
    
    two_count*three_count
}

fn find_prototype(input: &str) -> String {
    let mut same_letters = String::new();
    
    let words: Vec<&str> = input.split("\n").collect();
    let mut letters: Vec<Vec<char>> = Vec::new();

    for w in words.iter() {
        letters.push(w.chars().collect());
    }

    let mut found = false;

    while !found {
        let test_word = letters.remove(0);

        for l in letters.iter() {
            let mut count = 0;
            let mut matches = 0;
            let matching_letters: Vec<_> = test_word.iter().zip(l.iter())
                .filter_map(|(a,b)| {
                    if a!=b {
                        count+=1;
                        None
                    } else {
                        matches+=1;
                        Some(a)
                    }
                }).collect();

            if count ==1 {
                found = true;
                same_letters = matching_letters.into_iter().collect();

            }
            
        }
    }
    
    same_letters
}

fn main() {
    let puzzle_input = "cvfueihajytpmrdkgsxfqplbxn
cbzueihajytnmrdkgtxfqplbwn
cvzucihajytomrdkgstfqplqwn
cvzueilajytomrdkgsxfqwnbwn
cvzueihajytomrdkgsgwqphbwn
wuzuerhajytomrdkgsxfqplbwn
cyzueifajybomrdkgsxfqplbwn
cvzueihajxtomrdkgpxfqplmwn
ivzfevhajytomrdkgsxfqplbwn
cvzueihajytomrdlgsxfqphbbn
uvzueihajjtomrdkgsxfqpobwn
cvzupihajytomrdkgsxfqplpwe
cvzueihajyvomrdkgsxfqplbrl
cczueihajytomrdkgsnfqpxbwn
cvzueigajytdmrdkgsxyqplbwn
cvzujihljytomrdkgsxuqplbwn
cvzueisajytomrddgsxkqplbwn
cvzneihajytomrdkgsgaqplbwn
cvzueihajytomrdkgsinmplbwn
cveueihajyromrdkgsxfqplown
cypueihajytotrdkgzxfqplbwn
cvzuoihajytomvdqgsxfqplbwn
cvzuekhejytwmrdkgsxfqplbwn
cvzseihajytomrdkgsxfqgmbwn
cvfuhihajytomrdkgsxfqplbwi
cvzueihujxtomrdkgsufqplbwn
cvzueihdjytomrdogsxfqplbwh
cvzueihdjyfohrdkgsxfqplbwn
cvtudihajytolrdkgsxfqplbwn
cvzueihajytymrdkgshzqplbwn
cvzuebhajytomxdkgsxfqplbwt
cvzulihajyxomrdkgsbfqplbwn
cvzueihajywomrdkgsxfqplbts
cvzueihajytouodkdsxfqplbwn
cvzueihajytomgdkgqxfqklbwn
cvzubihajytomvdkgsxfqplmwn
cvhueihajyyocrdkgsxfqplbwn
zvzueihajytourdkgsxflplbwn
cvzbeihajytomadkgsxfoplbwn
cvzueihajytomrdkgnxfqplbsl
cvfueihajftkmrdkgsxfqplbwn
cvzuexhajytomryugsxfqplbwn
cvzueihajytomsckgsxfqalbwn
cvzuexhajytomrdkbsxfqpluwn
cvzueihajytbmrtkgsxwqplbwn
cvzueihajytomrdigsxfqqlbsn
cvzweihajytomydkgsxfmplbwn
bvzteihajytimrdkgsxfqplbwn
cvzueihajytpmrdkgsxfcpbbwn
cvzueigsjltomrdkgsxfqplbwn
cvzueihajytomrikgsxfopldwn
cvzueihajstomrdkgsxfqplgon
cvzueimajytomrnkxsxfqplbwn
cvzleihagatomrdkgsxfqplbwn
cvbueihajotomrdkgsxfqjlbwn
cvzueihajytomrdkgsxfqppnvn
hvzueihajytomrdkghxfkplbwn
cvzueigajytxmrdkgsxfqplbjn
cvzueihaaxtokrdkgsxfqplbwn
cvzueihajyeomrdkgujfqplbwn
cvzueiwajpoomrdkgsxfqplbwn
cvzieidtjytomrdkgsxfqplbwn
cvzueihalytomrakbsxfqplbwn
wtzueihajytomrdkgsxfqplbwq
cvzuelhaiytomrdkgsxfqplcwn
cvzueihajytomrdkgsxfqslswd
cvzueihajytomrykgssfqplbon
cvzueihfjytovrdegsxfqplbwn
cvzueihajytomldqgsxfqplbwy
cvzleihjjytomrtkgsxfqplbwn
cvzueihaldtomrdtgsxfqplbwn
cvzueihajytzmrdkgsxfeplqwn
cvzueihrjytomddkgsxfqpgbwn
cyzulihajytokrdkgsxfqplbwn
cvsueihajytoordfgsxfqplbwn
fvzueyhajytomrdkgaxfqplbwn
cczueihajytobrdkgsefqplbwn
cvzueihajytomcdrgscfqplbwn
cvzuexhajyvomrdkgssfqplbwn
cvzsmihajyiomrdkgsxfqplbwn
cvzzeihajttomrdkgsxzqplbwn
cvzseihajytomrdkgsxfqpebvn
cvzueihajgthmrdkgsbfqplbwn
ruzueihajytomrdkgsxfqphbwn
cvzueihajytofrdkgsnfrplbwn
cvzuetdajytojrdkgsxfqplbwn
fvzueihajytomrdkghxfqpobwn
cvzueihsjytomrdkgsxfqglbxn
cvzueihajytowrdkgsxfqpsbun
cvzteihaiytomrdkfsxfqplbwn
cvzueihajytkmrdkrsxfqplvwn
cvzueihajyoomrdkasxfqjlbwn
lvzurihajytkmrdkgsxfqplbwn
cvzueihajyyomrdagsxfqelbwn
cvfueihajytomrdkgsxfqplbbx
cvwueihajytommdkgkxfqplbwn
cvzucicajytomrdkgsxcqplbwn
dvzueihahytgmrdkgsxfqplbwn
cvzuechajytomrdkgsxfqelwwn
cvzuekhajytomrdkgsxknplbwn
cvtueihajytomphkgsxfqplbwn
cvzueihabytnzrdkgsxfqplbwn
cvzusihajytomrdkgfxfqplban
cvfueihajytomcdfgsxfqplbwn
mvzueihapytomrdkgsxfdplbwn
cvzueihajytomhdkgsxmqppbwn
jvsueihajytomrdkgsxfqplbln
cvzujihajybomrdkgsxtqplbwn
cvzuekhawytomrdkgsxfqplbwc
svzueihanytomrdogsxfqplbwn
cvzujihajytodrdkgslfqplbwn
cvgdeihajytorrdkgsxfqplbwn
cvzbeihajytoprdkgsxfqplbyn
cvzueihkyytomjdkgsxfqplbwn
cvzuelhojytomrdkgsxfqjlbwn
evzueihajytimrdkgsxfqpsbwn
cvzueihajydomrdkjsxfqplbjn
ovzteihajytosrdkgsxfqplbwn
cvzueihajyaomrdzgsxfqplbgn
cvzuewhajmtomrdkgsufqplbwn
cvzueihajqtomhukgsxfqplbwn
cvzueihajytomzqkgsxfqplbwk
cazuewhakytomrdkgsxfqplbwn
clzueihatytomrdkgzxfqplbwn
dvzueihajytomqdkgsxfqpnbwn
cvzueidajdtomrdkgsxtqplbwn
cvzueihabytowrdkgsxoqplbwn
cvzujihwjytomrdkgsxeqplbwn
cvtuedhajytomrdkgsxfqplbbn
cvzueihajcgomrdkgsxfqplswn
cvzuephajyiomrdngsxfqplbwn
cvzueihajythmqdkgsxfqplbwf
cvzueitajytomrdkgsxfepvbwn
cvzueihajytomydkgsxfqplvwb
dvzueshajytomrddgsxfqplbwn
cvzueihajytomrdkgvxfqpwben
cvzueihajytomrdkgvxfpplwwn
cvzuefhajftomrdkgsxfqrlbwn
cvzueihajytpmrvkgsxfqplbcn
cvzueihajytohrdkgsxfqxnbwn
cvzueihajytomrdposxfqulbwn
cozueihajytomrpkgsxfqrlbwn
cvzuuihaxytomrdkgsxfqplbtn
cvzueihajytomrbzgsxyqplbwn
cveueihajyxoqrdkgsxfqplbwn
cvzueihajytomrkkgsxfqptbrn
cvzuezhajatomrdkssxfqplbwn
cpzueihajytomrdkgsxfhplbwo
lviueihajytomrekgsxfqplbwn
cvzueihwjytomrdkusxfyplbwn
cvzgeihajytomwdkgsxfrplbwn
cvzsejhzjytomrdkgsxfqplbwn
cvzuuihajytomrdkgsxfqdlbwz
cvzjeihajytomrdugsxftplbwn
cvzueihaxytomrrkgsxfmplbwn
cvzueihajgtomrdhgsxfqplwwn
cvzulihajytomedkgsxfqplewn
cvzueivajytomrdkmsxfqplbwc
cvzuervajytomrdkgsxfwplbwn
cvzuemhcjytomrdkgslfqplbwn
cvzyerhauytomrdkgsxfqplbwn
cvzueihaoytomrdkgsyfqplewn
cvzueihanytomrdkgsafkplbwn
cvzueihajvtomrdugsxfqpcbwn
chzueihajytamrdxgsxfqplbwn
cvzueihalytomrdsgsxfqplbln
cvzueihajytoyaykgsxfqplbwn
tlzueihajyeomrdkgsxfqplbwn
cvpueihajytbmrdkgsxfxplbwn
cvzueihajytomjdkgsxuqplkwn
cvzueihajygomrdkgkxfqplbwg
cvzueihajhtomrdkgbxsqplbwn
cvzurihajytomrdkgsafqplbwx
cdzuezhajytomrdkgsxrqplbwn
cvbueihajytotrwkgsxfqplbwn
cwzkeihajytomrdkgsxfqplbwh
cvzheihajytolrikgsxfqplbwn
cozuevhajytomrdkgkxfqplbwn
chzueihajytomrjkgsxfqulbwn
cvzueihkjyromrdkgsxvqplbwn
cvzveihajytomrdkgsxpqplnwn
cvzueihajytoirdkgsxfqihbwn
cvoueihajytomrdkgsxfqpdawn
pvzueihajytomrdkgnxfqplbfn
cvzueihakytomxdkgssfqplbwn
cvzueivajytomrdbgsxaqplbwn
cvzueihajytokrdkgszrqplbwn
cvzuevhajytomrdkgsxgqplbwi
cvzueihajylomrdkgsxflplbpn
hvzueihajytomvdkgsxfqplgwn
cvzleihajytymrrkgsxfqplbwn
crzueieajytomrdkgsxfqplbon
cszueihajytomrdlgqxfqplbwn
cvzueihacytomrdkgsxfjblbwn
cvzreihajytomrdkgsxfqplzun
cvzurihajytomrdkgsxiqplawn
uvzueihajyhovrdkgsxfqplbwn
cvzueihajyqodrdkgssfqplbwn
cvzwiihrjytomrdkgsxfqplbwn
cqzueihajytomrdkgjxfqplban
cvmueihajytoordkgsxfqplbyn
cypueihajytomrdkgzxfqplbwn
cvzueihajykomrdkgsmfqplbtn
cvzueidajytimrdkgsxfqpdbwn
cvzheihajytomrdkgsxfqpfewn
dvzueihajytumrdzgsxfqplbwn
cvzueixajytomrdkgsvfqplgwn
cvzuevhzjyzomrdkgsxfqplbwn
cvyeeihajytomrdkgsxnqplbwn
cvzueihajytomrdkggtpqplbwn
cvzceiyajytomrdkgexfqplbwn
cvzuelhajyyomrdkzsxfqplbwn
cvzhzihajygomrdkgsxfqplbwn
cvzueihwjytomrdkgsgfqplbrn
cvzsevhajytomrdkgqxfqplbwn
cvzueiuajytomrdkgsxfppebwn
nvzueihajytemrdkgsxwqplbwn
cvzueihajytocgdkgsxfqvlbwn
cczusihajytomrdkgsxfqplbpn
cmzueihajytomrdkbsxwqplbwn
cvzumfdajytomrdkgsxfqplbwn
cvzueihcjytomrdkgsxfqplbkl
cvzueihajytomawknsxfqplbwn
kvzueihijytomrdkgsxdqplbwn
cdzutihajytomrdkgsxfkplbwn
cvzufihadylomrdkgsxfqplbwn
cvzueihajytomrgkxsxfqphbwn
cvzuewhajyzomrdkgsxfqelbwn
cvzueihajytomrdkgqxfqelbwc
cvzueshajyoomrdkgsxfqflbwn
cvzueihajyromrekgixfqplbwn
chzugihajytomrdkgsxfqplawn
cvzueihajytomrdkgsxfhpmbwy
cvzueihacytodxdkgsxfqplbwn
cvzurihajytourdkgsdfqplbwn
cvzzeihmjytomrddgsxfqplbwn
cvzucyhajygomrdkgsxfqplbwn
ckzueihzjytomrdkgsxwqplbwn
cvlueihajmtozrdkgsxfqplbwn
cvzkeihajytomrdkgsxfqclbwc
cvzueihajytomrdkgsxgdplbwa
cvzueihyjytoxrdkgcxfqplbwn
cvzueizavytomfdkgsxfqplbwn
cvzueihajwtosrdkgsxfqllbwn
cvzueihajytomrdaksxfqpllwn
cvzuuihojytombdkgsxfqplbwn
cvzuiibajytpmrdkgsxfqplbwn
cvzueihajyuomydkgsxfqplzwn
cvzueihajytimrmkgsxfqplfwn
cvzueihajytomrdkgzxfqpljwo";
    
    let parse = parse_letters(puzzle_input);
    println!("Part one: {}", calculate_checksum(parse));

    println!("Part two: {}", find_prototype(puzzle_input));
}
