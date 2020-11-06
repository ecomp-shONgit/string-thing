

use RUSTYFASTSTRINGTHING::textnorm;
use RUSTYFASTSTRINGTHING::textdecomp;
use RUSTYFASTSTRINGTHING::strdist;



fn main() {
    

    println!("Start running the rusty fast string thing.");
    let testit = true;

    if testit {
        let mima = String::from( "hh † „[IX]” ⁙  ἀλλ’ ἑτέραν τινὰ φύσιν ἄπειρον', ἐξ ἧς ἅπαντας γίνεσθαι τοὺς οὐρανοὺς καὶ τοὺς ἐν αὐτοῖς κόσμους· ἐξ ὧν δὲ ἡ γένεσίς ἐστι τοῖς οὖσι, καὶ τὴν φθορὰν εἰς ταῦτα γίνεσθαι κατὰ τὸ χρεών. διδόναι γὰρ αὐτὰ δίκην καὶ τίσιν ἀλλήλοις τῆς ἀδικίας κατὰ τὴν τοῦ χρόνου τάξιν, ποιητικωτέροις οὕτως ὀνόμασιν αὐτὰ λέγων· δῆλον δὲ ὅτι τὴν εἰς ἄλληλα μεταβολὴν τῶν τεττάρων στοιχείων οὗτος θεασάμενος οὐκ ἠξίωσεν ἕν τι τούτων ὑποκείμενον ποιῆσαι, ἀλλά τι ἄλλο παρὰ ταῦτα. οὗτος δὲ οὐκ ἀλλοιουμένου τοῦ στοιχείου τὴν γένεσιν ποιεῖ, ἀλλ’ ἀποκριν-\nομένων τῶν ἐναντίων διὰ τῆς ἀιδίου κινή- σεως· 1 Summá pecúniae, quam dedit in [bla bla bla] aerarium vel plebei Romanae vel dimissis militibus=> denarium sexiens milliens. 2 Opera fecit nova § aedem Martis, Iovis Tonantis et Feretri, Apollinis, díví Iúli, § Quirini, § Minervae, Iunonis Reginae, Iovis Libertatis, Larum, deum Penátium, § Iuventatis, Matris deum, Lupercal, pulvinar ad [11] circum, § cúriam cum chalcidico, forum Augustum, basilicam 35 Iuliam, theatrum Marcelli, § porticus . . . . . . . . . . , nemus trans Tiberím Caesarum. § 3 Refécit Capitolium sacrasque aedes numero octoginta duas, theatrum Pompeí, aquarum rivos, viam Flaminiam.  Ϗ ϗ ϚϛȢȣꙊꙋἀἁἂἃἄἅἆἇἈἉἊἋἌἍἎἏἐἑἒἓἔἕἘἙἚἛἜἝἠἡἢἣἤἥἦἧἨἩἪἫἬἭἮἯἰἱἲἳἴἵἶἷἸἹἺἻἼἽἾἿὀὁὂὃὄὅὈὉὊὋὌὍὐὑὒὓὔὕὖὗὙὛὝὟὠὡὢὣὤὥὦὧὨὩὪὫὬὭὮὯὰάὲέὴήὶίὸόὺύὼώ	ᾀᾁᾂᾃᾄᾅᾆᾇᾈᾉᾊᾋᾌᾍᾎᾏᾐᾑᾒᾓᾔᾕᾖᾗᾘᾙᾚᾛᾜᾝᾞᾟᾠᾡᾢᾣᾤᾥᾦᾧᾨᾩᾪᾫᾬᾭᾮᾯᾰᾱᾲᾳᾴᾶᾷᾸᾹᾺΆᾼ᾽ι᾿῀῁ῂῃῄῆῇῈΈῊΉῌ῍῎῏ῐῑῒΐῖῗῘῙῚΊ῝῞῟ῠῡῢΰῤῥῦῧῨῩῪΎῬ῭΅`ῲῳῴῶῷῸΌῺΏῼ´῾ͰͱͲͳʹ͵Ͷͷͺͻͼͽ;Ϳ΄΅Ά·ΈΉΊΌΎΏΐΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩΪΫάέήίΰαβγδεζηθικλμνξοπρςστυφχψωϊϋόύώϏϐϑϒϓϔϕϖϗϘϙϚϛϜϝϞϟϠϡϢϣϤϥϦϧϨϩϪϫϬϭϮϯϰϱϲϳϴϵ϶ϷϸϹϺϻϼϽϾϿ Αι αι γγ γκ γξ γχ ου Υι υι ἄϋλος αὐλός  τί φῄς; γραφὴν σέ τις, ὡς ἔοικε, γέγραπται οὐ γὰρ ἐκεῖνό γε καταγνώσομαι, ὡς σὺ ἕτερον. δ̣[ὲ κ]αὶ?");
        let atesttext = textnorm::normatextNFD( &mima );
        
        println!("RUN TESTS");
        println!("-- Test: isnumber fn");
        let anum = String::from("aber");
        println!("{} {}", anum, textnorm::isnumber(&anum));
        let bnum = String::from("100");
        println!("{} {}", bnum, textnorm::isnumber(&bnum));
        let cnum = String::from("14c");
        println!("{} {}", cnum, textnorm::isnumber(&cnum));
        let dnum = String::from("lxxxiv");
        println!("{} {}", dnum, textnorm::isnumber(&dnum));
        let eenum = String::from("νδ");
        println!("{} {}", eenum, textnorm::isnumber(&eenum));

       
        println!("-- Test: disambiguDIAkritika fn {}\n\n", textnorm::disambiguDIAkritika(&atesttext));   
        println!("-- Test: sigmaistgleich fn {}\n\n", textnorm::sigmaistgleich(&atesttext));
        println!("-- Test: deluv fn {}\n\n", textnorm::deluv(&atesttext)); 
        println!("-- Test: sameallspacing fn {}\n\n", textnorm::sameallspacing(&atesttext));
        println!("-- Test: disambiguadashes fn {}\n\n", textnorm::disambiguadashes(&atesttext));   
        println!("-- Test: value textdecomp {}\n\n", textdecomp::avalue);
        let kleinwo = String::from("῾ἑτέραν");
        println!("-- Test: ExtractDiafromBuchst fn {:?}\n\n", textnorm::ExtractDiafromBuchst(&kleinwo));
        println!("-- Test: ExtractDiafromBuchstText fn {:?}\n\n", textnorm::ExtractDiafromBuchstText(&atesttext));
        println!("-- Test: replaceBehauchung fn {:?}\n\n", textnorm::replaceBehauchung(&kleinwo));
        let elu = String::from("ἀλλ’");
        println!("-- Test: Expandelision fn {:?}\n\n", textnorm::Expandelision(&elu));
        println!("-- Test: ExpandelisionText fn {:?}\n\n", textnorm::ExpandelisionText(&atesttext));
        let lilalat = String::from("Aberja");
        println!("-- Test: TranslitLatinGreekLetters fn {}\n\n", textnorm::TranslitLatinGreekLetters( &lilalat ) );
        println!("-- Test: TraslitAncientGreekLatin fn {}\n\n", textnorm::TraslitAncientGreekLatin( &elu ) );
        let spitzeexa = String::from("<aberja>");
        println!("-- Test: spitzeklammernHTML fn {}\n\n", textnorm::spitzeklammernHTML( &spitzeexa ) );
        println!("-- Test: basClean fn {}\n\n", textnorm::basClean( &atesttext ) );

        println!("-- Test: tennsatzei fn {}\n\n", textnorm::tennsatzei( &atesttext ) ); 
        let singlealpha = textnorm::normatextNFD( &"ἄϋλος".to_string() );
        println!("-- Test: AlphaPrivativumCopulativum fn {}\n\n", textnorm::AlphaPrivativumCopulativum( &singlealpha ) ); 
        println!("-- Test: AlphaPrivativumCopulativumText fn {}\n\n", textnorm::AlphaPrivativumCopulativumText( &atesttext ) ); 
        println!("-- Test: ohnediakritW fn {}\n\n", textnorm::ohnediakritW( &atesttext ) ); 
        println!("-- Test: capitali fn {}\n\n", textnorm::capitali( &singlealpha ) ); 
        println!("-- Test: nodiakinword fn {}\n\n", textnorm::nodiakinword( &singlealpha ) );  
        println!("-- Test: delall fn {}\n\n", textnorm::delall( &atesttext ) ); 
        println!("-- Test: delnumbering fn {}\n\n", textnorm::delnumbering( &atesttext ) ); 
        println!("-- Test: deldiak fn {}\n\n", textnorm::deldiak( &atesttext ) ); 
        println!("-- Test: delinterp fn {}\n\n", textnorm::delinterp( &atesttext ) ); 
        println!("-- Test: delunknown fn {}\n\n", textnorm::delunknown( &atesttext ) );  
        println!("-- Test: delumbrbine fn {}\n\n", textnorm::delumbrbine( &atesttext ) ); 
        println!("-- Test: umbrtospace fn {}\n\n", textnorm::umbrtospace( &atesttext ) );  
        let mymarkymark = String::from("<div id=klkl>JJsush </div>");
        println!("-- Test: delmakup fn {}\n\n", textnorm::delmakup( &mymarkymark ) ); 
        println!("-- Test: makuptoleer fn {}\n\n", textnorm::makuptoleer( &mymarkymark ) ); 
        println!("-- Test: delgrkl fn {}\n\n", textnorm::delgrkl( &atesttext ) );  
        println!("-- Test: delklammern fn {}\n\n", textnorm::delklammern( &atesttext ) );  
        println!("-- Test: deledklammern fn {}\n\n", textnorm::deledklammern( &atesttext ) );  
        let little = String::from("ἀλλ’ ἀποκριν-\nομένων τῶν κινή- σεως· 1 Summá alles- [22]\nklar");
        let mywlistl: Vec<&str> = little.split( " " ).collect( );
        println!("-- Test: Trennstricheraus fn {}->{:?}\n\n", little, textnorm::Trennstricheraus( &mywlistl ) ); 
        println!("-- Test: UmbruchzuLeerzeichen fn {}\n\n", textnorm::UmbruchzuLeerzeichen( &atesttext ) ); 
        let mywlist: Vec<&str> = atesttext.split( " " ).collect( ); 
        println!("-- Test: Interpunktiongetrennt fn {:?}\n\n", textnorm::Interpunktiongetrennt( &mywlist ) ); 
        println!("-- Test: iotasubiotoadL fn {:?}\n\n", textnorm::iotasubiotoadL( &mywlist ) );
        println!("-- Test: GRvorbereitungT fn {:?}\n\n", textnorm::GRvorbereitungT( &atesttext ) ); 
        let befco = String::from(" Παῦν̣ι̣ κ");
        let littleunterpunkted = textnorm::normatextNFD( &befco );
        println!("-- Test: delUnterpunkt fn {}->{}\n\n", littleunterpunkted, textnorm::delUnterpunkt( &littleunterpunkted ) ); 
        


    
    }
    println!("End.");

}