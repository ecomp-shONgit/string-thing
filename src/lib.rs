//******************************************************************************
//
// 2020 norm, decomposition and transformation of string repres text 
// Rust (Lib) Modul, 
// Prof. Charlotte Schubert Alte Geschichte, Leipzig
//
//******************************************************************************



/*
GPLv3 copyrigth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//rust compiler config
#![recursion_limit="256"]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

//crates
extern crate regex;
extern crate unic_normal;
extern crate lazy_static;


//******************************************************************************
// MOD STRINGDIST
// 2020 string distances Rust (Lib) Modul, 
// Prof. Charlotte Schubert Alte Geschichte, Leipzig
//******************************************************************************
pub mod strdist{
    use std::cmp;
    use std::collections::HashSet;
    use std::collections::HashMap;

    /*------------------------------------------------------------------------------
                     Programming Helper
    ------------------------------------------------------------------------------*/
    pub fn set<'a>( aA: &'a Vec<&str> ) -> HashSet<&'a str> {
        //return aA.into_iter( ).collect( ); //move
        return aA.iter( ).cloned( ).collect( ); //copy
    }

    pub fn list<'a>( aS: &'a HashSet<&str> ) -> Vec<&'a str> {
        //return aS.into_iter( ).collect( ); //move
        return aS.iter( ).cloned( ).collect( ); //copy
    }

    fn max<T:cmp::Ord>( a: T, b: T ) -> T {
        return cmp::max( a, b );
    }

    fn min<T:cmp::Ord>( a: T, b: T ) -> T {
        return cmp::min( a, b );
    }

    fn pmax<T:cmp::PartialOrd>( a: T, b: T ) -> T {
        if a > b {
            return a;
        } else {
            return b;
        }
    }

    fn pmin<T:cmp::PartialOrd>( a: T, b: T ) -> T {
        if a < b {
            return a;
        } else {
            return b;
        }
    }
    
    static True: bool = true;
    static False: bool = false;

    /*------------------------------------------------------------------------------
            SET OPERATIONS 
    ------------------------------------------------------------------------------*/

    pub fn SetSymDiff<'a>( setA: &HashSet<&'a str>, setB: &HashSet<&'a str> ) -> HashSet<&'a str> { 
        let retset: HashSet<_> = setA.symmetric_difference( setB ).cloned( ).collect( );
        return retset;
    }

    pub fn SetDiff<'a>( setA: &HashSet<&'a str>, setB: &HashSet<&'a str> ) -> HashSet<&'a str> { 
        return setA.difference( setB ).cloned( ).collect( );
    }

    pub fn SetUnsion<'a>( setA: &HashSet<&'a str>, setB: &HashSet<&'a str> ) -> HashSet<&'a str> { 
        return setA.union( setB ).cloned( ).collect( );
    }

    pub fn SetIntersection<'a>( setA: &HashSet<&'a str>, setB: &HashSet<&'a str> ) -> HashSet<&'a str> { 
        return setA.intersection( setB ).cloned( ).collect( );
    }

    pub fn SetSymDiffLen( setA: &HashSet<&str>, setB: &HashSet<&str> ) -> usize { 
        return setA.symmetric_difference( setB ).collect::<HashSet<_>>( ).len( );
    }

    pub fn SetDiffLen( setA: &HashSet<&str>, setB: &HashSet<&str> ) -> usize { 
        return setA.difference( setB ).collect::<HashSet<_>>( ).len( );
    }

    pub fn SetUnsionLen( setA: &HashSet<&str>, setB: &HashSet<&str> ) -> usize { 
        return setA.union( setB ).collect::<HashSet<_>>( ).len( );
    }

    pub fn SetIntersectionLen( setA: &HashSet<&str>, setB: &HashSet<&str> ) -> usize { 
        return setA.intersection( setB ).collect::<HashSet<_>>( ).len( );
    }
    
    /*------------------------------------------------------------------------------
            generalized comparison: DISTANCES
    ------------------------------------------------------------------------------*/

    //NOTE: all dist/Containedness/common functions take two arrays as first input, the array could be any representation of text (string, sequence, gram, selected words)

    pub fn WLEV( s1: &Vec<&str>, s2: &Vec<&str>, Wv: &Vec<usize>, Ws: &HashMap<String, usize> ) -> usize { 
        /*
            NAME: weighted levenshtein, 
            INPUT: - s1 and s2 as representations, 
                   - Wv a weight for pairs in A and B, 
                   - Ws a list of 4 weights related to the operations 
                     substitution, insertion, deletion, exchange,
            RETURN: Number of edited Letters / sum of editweights,

        */
        //println!("s1 {:?}\n\n", s1);
        //println!("s2 {:?}\n\n", s2);
        let lens1 = s1.len( );
        let lens2 = s2.len( );
        
        //println!("lens1 {}\n\n", lens1);
        //println!("lens2 {}\n\n", lens2);
        if lens1 == 0 || lens2 == 0 { 
            return usize::MAX;
        }
         
        if lens1 < lens2 {
            return WLEV( s2, s1, Wv, Ws );
        }
        
        let mut m: Vec<Vec<usize>> = vec![]; // is matrix
        
        // increment along the first column of each row
        for i in 0..lens2 {
            let mut insvec = vec![0; lens1 ];
            insvec[0] = i;
            m.push( insvec );
        }
        // increment each column in the first row
        for j in 0..lens1 {
          m[ 0 ][j] = j;
        }
        //println!("m {:?}\n\n", m);
        // fill in the rest of the matrix
        for i in 1..lens2 {
          //println!("i {}", i);
          for j in 1..lens1 {

            if s2[ i-1 ] == s1[ j-1 ] {
              m[i][j] = m[i-1][j-1];
                
            } else {
                let charsum = "".to_string()+s2[ i-1 ]+s1[ j-1 ];
                let mut weightofdigram = 0;
                if Ws.contains_key( &charsum ) {
                    weightofdigram = Ws[ &charsum ];
                }
                if 1 < i && 1 < j {
                    m[i][j] = min( 
                                min(
                                    m[i-1][j-1] + Wv[0], //substitution
                                    min(
                                        m[i][j-1] + Wv[1], //insertion
                                        m[i-1][j] + Wv[2])), //deletion
                                m[i-2][j-2] + Wv[3] ) //exchange
                             + weightofdigram; //digram weight
                } else {
                    m[i][j] = min(m[i-1][j-1] + Wv[0], // substitution
                        min(m[i][j-1] + Wv[1], // insertion
                        m[i-1][j] + Wv[2])) // deletion
                            + weightofdigram; //digram weight
                }
            }
          }
        }
        //println!("m {:?}\n\n", m);
        return m[ lens2-1 ][ lens1-1 ]; //returns distnace similarity is 1 - (d/max(len(A,B)))
    }//END WLEV

    pub fn LEVDAM( s1: &Vec<&str>, s2: &Vec<&str>, Wv: &Vec<usize> ) -> usize { 
        /*
            NAME: damerau levenshtein,
            INPUT: - a text representation s1 and s2,
                   - Wv a list of 4 weights related to the operations 
                     substitution, insertion, deletion, exchange,
            RETURN: sum of editweights,

        */
        let lens1 = s1.len( );
        let lens2 = s2.len( );
        if lens1 == 0 || lens2 == 0 { 
            return usize::MAX;
        }
         
        if lens1 < lens2 {
            return LEVDAM( s2, s1, Wv );
        }
        
        let mut m: Vec<Vec<usize>> = vec![]; // is matrix
        
        //increment along the first column of each row
        for i in 0..lens2 {
            let mut insvec = vec![0; lens1 ];
            insvec[0] = i;
            m.push( insvec );
        }
        //increment each column in the first row
        for j in 0..lens1 {
          m[ 0 ][j] = j;
        }
        //fill in the rest of the matrix
        for i in 1..lens2 {
          for j in 1..lens1 {

            if s2[ i-1 ] == s1[ j-1 ] {
              m[i][j] = m[i-1][j-1];
                
            } else {
                if 1 < i && 1 < j {
                    m[i][j] = min( 
                                min(
                                    m[i-1][j-1] + Wv[0], //substitution
                                    min(
                                        m[i][j-1] + Wv[1], //insertion
                                        m[i-1][j] + Wv[2])), //deletion
                                m[i-2][j-2] + Wv[3] ); //exchange
                             
                } else {
                    m[i][j] = min(m[i-1][j-1] + Wv[0], // substitution
                        min(m[i][j-1] + Wv[1], // insertion
                        m[i-1][j] + Wv[2])); // deletion
                }
            }
          }
        }
        return m[ lens2-1 ][ lens1-1 ]; //returns distnace similarity is 1 - (d/max(len(A,B)))
    } //END LEVDAM

    pub fn levenshtein( s1: &Vec<&str>, s2: &Vec<&str>, Wv: &Vec<usize> ) -> usize { 
        /*
            NAME: Levenshtein wie immer, weightable,
            INPUT: - s1 and s2 text representations,
                   - Wv a list of 4 weights related to the operations 
                     substitution, insertion, deletion, exchange,
            RETURN: number of edits,
        */
        let lens1 = s1.len( );
        let lens2 = s2.len( );
        if lens1 == 0 || lens2 == 0 { 
            return usize::MAX;
        }
         
        if lens1 < lens2 {
            return LEVDAM( s2, s1, Wv );
        }
        
        let mut m: Vec<Vec<usize>> = vec![]; // is matrix
        
        //increment along the first column of each row
        for i in 0..lens2 {
            let mut insvec = vec![0; lens1 ];
            insvec[0] = i;
            m.push( insvec );
        }
        //increment each column in the first row
        for j in 0..lens1 {
          m[ 0 ][j] = j;
        }
        //fill in the rest of the matrix
        for i in 1..lens2 {
          for j in 1..lens1 {

            if s2[ i-1 ] == s1[ j-1 ] {
              m[i][j] = m[i-1][j-1];
                
            } else {
                m[i][j] = min(
                                m[i-1][j-1] + Wv[0], // substitution
                                min(
                                    m[i][j-1] + Wv[1], // insertion
                                    m[i-1][j] + Wv[2]
                                    )
                              ); // deletion
            }
          }
        }
        return m[ lens2-1 ][ lens1-1 ]; //returns distnace similarity is 1 - (d/max(len(A,B)))
    } //END LEVENSHTEIN 

    pub fn LCS( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> usize { 
        /*
            NAME: longest common subsequence (sequence is not substring, it is like sequencial but not next to eachother),
            INPUT: vecA and vecB text representations,
            RETURN: 0 (distant) and  max(len(A),len(B)) (not distant),
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return 0; 
        }
        let mut C: Vec<Vec<usize>> = vec![vec![0; lenB ]; lenA ];
        for i in 0..lenA {
            for j in 0..lenB {
                if vecA[i] == vecB[j] {
                    if i != 0 && j != 0 {
                        C[i][j] = max( max( C[i][j-1]+1, C[i-1][j]+1 ), C[i-1][j-1] + 1);
                    } else {
                        C[i][j] = 1;
                    }
                } else {
                    if i != 0 && j != 0 {
                        C[i][j] = max( C[i][j-1], C[i-1][j] ); 
                    }
                }
            }
        }
        return C[lenA-1][lenB-1]; //SEE containedness: LCS/len(A) for B contained A or LCS/len(B) for A contained B
    } //END LCS
    
    pub fn LCF( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> usize { 
        /*
            NAME: longest common substring (factor, sequential and next to each other members of a vector),
            INPUT: vecA and vecB text representations,
            RETURN: 0 (distant, nothing in common) and  max(len(A),len(B)) (not distant),
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return 0; 
        }
        
        let mut C: Vec<Vec<usize>> = vec![vec![0; lenB ]; lenA ];
        let mut maxlen: usize = 0;
        for i in 0..lenA {
            for j in 0..lenB {
                if vecA[i] == vecB[j] {
                    if i != 0 && j != 0 {
                        C[i][j] = C[i-1][j-1] + 1;
                        if maxlen < C[i][j] {
                            maxlen = C[i][j];
                        }
                    } else {
                        C[i][j] = 1;
                    }
                } else {
                    if i != 0 && j != 0 {
                        if maxlen < C[i-1][j-1] {
                            maxlen = C[i-1][j-1];  
                        }
                    }
                    C[i][j] = 0;
                }
            }
        }
        
        return maxlen; 
    } //END LCF
    
    pub fn containednessLCS( a: &Vec<&str>, b: &Vec<&str> ) -> f64 {
        /*
            NAME: according to LCS the containedness of a in b or b in a,
            INPUT: a and b text representations,
            RETURN: 1 (contained) and 0 (not contained),
        */
        let lenb = b.len( );
        let lena = a.len( );
        if lena == 0 || lenb == 0 {
            return 0.0;
        }
        let lcsab = LCS( a,b );
        //print!("LCS {}, lena {}, lenb {}, diva {}, divb {}\n", lcsab, lena, lenb,lcsab/lena, lcsab/lenb);
        if lcsab == 0 {
            return 0.0;
        } else {
            return pmax(  lcsab as f64 / lena as f64,  lcsab as f64 /  lenb as f64 );
        }
    } //END CONTAINEDNESSLCS

    pub fn containednessLCF( a: &Vec<&str>, b: &Vec<&str> ) -> f64 {
        /*
            NAME: according to LCF the containedness of a in b or b in a
            INPUT: a and b text representations
            RETURN: 1 (contained) and 0 (not contained),
        */
        let lenb = b.len( );
        let lena = a.len( );
        if lena == 0 || lenb == 0 {
            return 0.0;
        }
        let lcfab = LCF( a, b );
        if lcfab == 0 {
            return 0.0;
        } else {
            return pmax(  lcfab as f64 / lena as f64,  lcfab as f64 /  lenb as f64 );
        }
    } //END CONTAINEDNESSLCF

    pub fn LCP( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> usize {
        /*
            NAME: longest commen prefix,
            INPUT: vecA and vecB text representations,
            RETURN: 0 (distant) and  max(len(A),len(B)) (not distant),
        */
        let mut sizeofcommenprefix: usize = 0;
        let lenMIN = min( vecA.len( ), vecB.len( ) );
        if lenMIN == 0 { 
            return 0; 
        }
        
        for i in 0..lenMIN {
            if vecA[i] == vecB[i] {
                sizeofcommenprefix += 1;
            } else {
                break;
            }
        }
        return sizeofcommenprefix;
    } //END LCP

    pub fn bagdist( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> usize {
        /*
            NAME: bag distance (vecA is a bag is a sequencial, and next to eachother, redundant vector), aproximation of levensthein,
            INPUT: vecA and vecB text representations,
            RETURN: max(len(A),len(B)) (distant) and 0 (not distant),
        */
        let mut eraseA = vecA.clone( );
        let lenA = vecA.len( );
        let mut eraseB = vecB.clone( );
        let lenB = vecB.len( );
        
        for i in 0..lenA {
            match vecB.iter( ).position( |&p| p == vecA[ i ] ) {
                Some( v ) => eraseB[ v ] = "NONONO",
                None => (), 
            }
        }

        let mut countinB = 0;
        for i in 0..lenB {
            if eraseB[ i ] == "NONONO" {
                countinB += 1;
            }
        }

        for i in 0..lenB {
            match vecA.iter( ).position( |&p| p == vecB[ i ] ) {
                Some( v ) => eraseA[ v ] = "NONONO",
                None => (),
            }
        }

        let mut countinA = 0;
        for i in 0..lenA {
            if eraseA[ i ] == "NONONO" {
                countinA += 1;
            }
        }
        
        return max( countinA, countinB );
    } //END BAGDIST

    pub fn JA( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 { 
        /* 
            NAME: jaro distance,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf (distant) and 0.0 (not distant) ???,
        */

        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }
        if lenB < lenA {
            return JA( vecB, vecA );
        }
        let maAB: i64 = max( lenB, lenA ) as i64;

        /*if( lenA != lenB ){
            return maAB; //ist das der richtige rückgabewert
        }HÄ ????????????*/
        
        let matchDist: i64 = (maAB/2)-1;
        let mut Amatches: Vec<bool> = vec![false; lenA ];  
        let mut Bmatches: Vec<bool> = vec![false; lenB ];
        let mut matchcount: usize = 0;
        let mut traspocount: usize = 0;

        
        for i in 0..lenA {
            let sta = max( 0 as i64, i as i64 - matchDist ) as usize;
            let en =  min( i as i64 + matchDist + 1, lenB as i64 ) as usize;

            for j in sta..en {
                if Bmatches[j] {
                    continue;
                }
                if vecA[i] != vecB[j] {
                    continue;
                }
                Amatches[i] = true;
                Bmatches[j] = true;
                matchcount+=1;
                break;
            }
        }
        if matchcount == 0 {
            return maAB as f64; //ist das der richtige rückgabewert 
        }

        let mut j: usize = 0;
        for i in 0..lenA {
            if !Amatches[i] {
                continue;
            }
            while !Bmatches[j] {
                j+=1;
            }
            if vecA[i] != vecB[i] {
                traspocount += 1;
            }
            j+=1;
        }
        return  ( (matchcount as f64/lenA as f64) + (matchcount as f64/lenB as f64) + (((matchcount as f64 -traspocount as f64)/2.0)/matchcount as f64 )) / 3.0 ;  
    } //END JA

    pub fn JAWI( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 { 
        /* 
            NAME: jaro winkler distance, transpositions,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY;
        }
        let onlyJaro = JA( vecA, vecB );
        return onlyJaro + ( ( max( 4, LCP(vecA, vecB)) as f64 / 10.0 ) * ( 1.0 - onlyJaro ));
    } // END JAWI

    pub fn baire( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64  {
        /* 
            NAME: baire distance,
            INPUT: vecA, vecB text represenations,
            RETURN: 0 (distant) and 1.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }
        return  1.0 / (1.0 + LCP(vecA, vecB) as f64);
    }

    pub fn generalizedcantor( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64  {
        /* 
            NAME: gen. cantor distance, 
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }
        return f64::powf( 1.0/std::f64::consts::E, 1.0 + LCP(vecA, vecB) as f64 ); //a 1/Math.E can also be 1/2
    } // END GENERALIZEDCANTOR

    pub fn notgeneralizedcantor( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64  {
        /* 
            NAME: not gen. cantor distance, 
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }
        return f64::powf( 1.0/std::f64::consts::E, 1.0 + LCF(vecA, vecB) as f64 ); 
    }// END NOTGENERALIZEDCANTOR

    pub fn jaccardMASZzwei( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64  {
        /* 
            NAME: derived from jaccard distance, transpositions,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (not distant) and 0.0 (distant) ???,
        */ 
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }
        let setA = set( vecA );
        let setB = set( vecB );
        return  1.0 - ( SetSymDiffLen( &setA, &setB ) as f64  / SetUnsionLen( &setB, &setA ) as f64 );
    } // END JACCARDMASZZWEI

    pub fn jaccardMASZ( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64  {
        /* 
            NAME: jaccard distance, transpositions,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (distant) and 0.0 (not distant) ???,
        */ 
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY;  
        }
        let setA = set( vecA );
        let setB = set( vecB );
        return  1.0 -  ( SetIntersectionLen( &setA, &setB ) as f64  / SetUnsionLen( &setB, &setA ) as f64 );
    } // END JACCARDMASZ

    pub fn cosineMASZ( vecA: &Vec<&str>, vecB: &Vec<&str> )  -> f64  { 
        /* 
            NAME: cosine distance,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        }  
        //müsste doch so klappen, oder was????
        let setA = set( vecA );
        let setB = set( vecB );
        let setAB = SetUnsion( &setA, &setB );
        let unionAB = list( &setAB );

        //occurenz count of gram in A or B
        let mut x: Vec<usize> = vec![]; //A
        let mut y: Vec<usize> = vec![]; //B
        let lenAB = unionAB.len( );

        for i in 0..lenAB {
            let mut currcount: usize = 0;
            for j in 0..lenA {
                if unionAB[ i ] == vecA[ j ] {
                    currcount += 1;
                }
            }
            x.push( currcount );
            currcount = 0;
            for j in 0..lenB {
                if unionAB[ i ] == vecB[ j ] {
                    currcount += 1;
                }
            }
            y.push( currcount );
        }   
        let mut summederquadrateA: usize = 0;
        let mut summederquadrateB: usize = 0;
        let mut scalarprod: usize = 0;
        let lenx = x.len( );

        for u in 0..lenx {
            summederquadrateA += x[ u ] * x[ u ]; 
            summederquadrateB += y[ u ] * y[ u ];
            scalarprod += x[ u ] * y[ u ];
        }
    
        let vecnormA = f64::sqrt( summederquadrateA as f64 );
        let vecnormB = f64::sqrt( summederquadrateB as f64 );
    
        return 1.0 - ( scalarprod as f64 / ( vecnormA*vecnormB ) ); 
    } 

    pub fn quadradiffMASZ( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 { 
        /* 
            NAME: quadratic difference distance,
                  # vec A and B are arrays of ngrams or silben, quadraDiff is a messure taken from the haufigkeitsvektor of A and B
            INPUT: vecA, vecB text represenations,
            RETURN: Inf (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        } 
        //müsste doch so klappen, oder was????
        let setA = set( vecA );
        let setB = set( vecB );
        let setunion = SetUnsion( &setA, &setB );
        let unionAB = list( &setunion );

        //occurenz count of gram in A or B
        let mut x: Vec<usize> = vec![]; //A
        let mut y: Vec<usize> = vec![]; //B  
        let lenAB = unionAB.len( );
        for i in 0..lenAB {
            let mut currcount = 0;
            for j in 0..lenA {
                if unionAB[ i ] == vecA[ j ] {
                    currcount += 1;
                }
            }
            x.push( currcount );
            currcount = 0;
            for j in 0..lenB {
                if unionAB[ i ] == vecB[ j ] {
                    currcount += 1;
                }
            }
            y.push( currcount );
        }   
        let mut sumitup: f64 = 0.0;
        let lenx = x.len( );
        for u in 0..lenx {
            sumitup += (((x[ u ] - y[ u ]) * (x[ u ] - y[ u ])) as f64).abs( );
        }
        return f64::sqrt( sumitup );
    } // END QUADRADIFFMASZ

    pub fn diceMASZ( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 {
        /* 
            NAME: dice coefficent distance,
            INPUT: vecA, vecB text represenations,
            RETURN: Inf/1 (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        } 
        let setA = set( vecA );
        let setB = set( vecB );
        return 1.0 - ( ( 2.0 * SetIntersectionLen( &setA, &setB ) as f64 ) / ( setA.len( ) + setB.len( ) ) as f64 )
    } // END DICEMASZ

    pub fn markingmetric( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 {
        /* 
            NAME: marking distance,
                  # https://www.sciencedirect.com/science/article/pii/0166218X88900765
                  # wir untersuchen die Übergränge ist eine übergang nicht Teil des anderen, dann merke die position des buchstabens der in gelöscht werden muss, entweder einer oder beide
            INPUT: vecA, vecB text represenations,
            RETURN: Inf (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        } 
        let mut posesA: Vec<usize> = vec![];
        
        for i in 1..lenA {
            let mut iba: i64 = -1; 
            match vecB.iter( ).position( |&p| p == vecA[i-1] ) {
                Some( v ) => iba = v as i64,
                None => (), 
            }
            let mut ibb: i64 = -1;
            match vecB.iter( ).position( |&p| p == vecA[i] ) {
                Some( v ) => ibb = v as i64,
                None => (), 
            }
            if iba != -1 && ibb != -1 {
                if !( (iba-ibb).abs( ) == 1 ) {
                    posesA.push( i ); //völlig egal welcher index aufgeschrieben wird
                }
            } else {
                if iba == -1 && ibb == -1 {
                    posesA.push( i-1 );
                    posesA.push( i );
                } else {
                    posesA.push( i-1 );
                }
            }
        }

        let mut posesB: Vec<usize> = vec![];

        for i in 1..lenB {
            let mut iaa: i64 = -1;
            match vecA.iter( ).position( |&p| p == vecB[i-1] ) {
                Some( v ) => iaa = v as i64,
                None => (), 
            }
            let mut iab: i64 = -1;
            match vecA.iter( ).position( |&p| p == vecB[i] ) {
                Some( v ) => iab = v as i64,
                None => (), 
            }

            if iaa != -1 && iab != -1 {
                if !( (iaa-iab).abs( ) == 1 ) {
                    posesB.push( i ); //völlig egal welcher index aufgeschrieben wird
                }
            } else {
                if iaa == -1 && iab == -1 {
                    posesB.push( i-1 );
                    posesB.push( i );
                } else {
                    posesB.push( i-1 );
                }
            }
        }
        
        return ( ( posesA.len( )+1 ) as f64 * ( posesB.len( )+1 ) as f64 ).log( std::f64::consts::E );
    } // MARKINGMETRIC

    pub fn setdiffmetric( vecA: &Vec<&str>, vecB: &Vec<&str> ) -> f64 {
        /* 
            NAME: set diff distance, derived from marking metric, containedness gedanken
            INPUT: vecA, vecB text represenations,
            RETURN: Inf (distant) and 0.0 (not distant) ???,
        */
        let lenA = vecA.len( );
        let lenB = vecB.len( );
        if lenA == 0 || lenB == 0 { 
            return f64::INFINITY; 
        } 
        let setA = set( vecA );
        let setB = set( vecB );
        let ABlen = SetDiffLen( &setA, &setB );
        let BAlen = SetDiffLen( &setB, &setA );
        return ( ( ABlen+1 ) as f64 * ( BAlen+1 ) as f64 ).log( std::f64::consts::E );
    } // END SETDIFFMETRIC
}

//******************************************************************************
// MOD TEXTDECOMP
// 2020 text decomposition Rust (Lib) Modul, 
// Prof. Charlotte Schubert Alte Geschichte, Leipzig
//******************************************************************************
pub mod textdecomp{
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use unic_normal::StrNormalForm;
    
    pub static avalue: &str = "JUST A TEST IF STATIC IS IMPORTED FROM MODUL.";

       
    lazy_static! {
        
    //pub static ref satzzeichen: Vec<String> = vec![String::from("."), String::from(";"), String::from(","), String::from(":"), String::from("!"), String::from("?"), String::from("·")]; 
    pub static ref vokaleGRIS: HashMap<String, bool> = {
        let mut vokaleGRI = HashMap::new();
        vokaleGRI.insert( "ι".nfd().collect::<String>(),true);
        vokaleGRI.insert( "υ".nfd().collect::<String>(),true);
        vokaleGRI.insert( "ε".nfd().collect::<String>(),true);
        vokaleGRI.insert( "ο".nfd().collect::<String>(),true);
        vokaleGRI.insert( "α".nfd().collect::<String>(),true);
        vokaleGRI.insert( "ω".nfd().collect::<String>(),true); 
        vokaleGRI.insert( "η".nfd().collect::<String>(),true);
        return vokaleGRI;
    };

    pub static ref buchstLATS: HashMap<String, String> = {
        let mut buchstLAT = HashMap::new();
        buchstLAT.insert( "d".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("b".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("g".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("p".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("t".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("c".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("k".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("q".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("qu".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("ph".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("th".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("ch".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("x".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("z".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("f".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("v".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("s".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("m".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("n".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("l".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("r".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("a".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("i".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("e".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("o".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("u".nfd().collect::<String>(), "".to_string());  
        buchstLAT.insert("v".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("y".nfd().collect::<String>(), "".to_string());   
        buchstLAT.insert("h".nfd().collect::<String>(), "".to_string());  
        buchstLAT
    };
    }//lazy static end
}//end mod textdecomp



//******************************************************************************
// MOD TEXTNORM
// 2020 text normalisation Rust (Lib) Modul, 
// Prof. Charlotte Schubert Alte Geschichte, Leipzig
//******************************************************************************
pub mod textnorm{
    use crate::textdecomp; //crate means this crate
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use regex::Regex;
    use unic_normal::StrNormalForm;
    
    
    //GLOBALS
    static doUVlatin: bool = false; //configure this true to use a latin u v conversion
    //static analysisNormalform: &'static str = "NFKD";
    //static dispnormalform: &'static str = "NFC";

    lazy_static! {
    //letters and words arrays
    static ref notprivalphaS: HashMap<String, String> = {
        let notprivalpha = HashMap::new();//["ἀΐω"];
        return notprivalpha;
    };

    //letters and words arrays
    static ref buchstGRIS: HashMap<String, String> = {
        let mut buchstGRI = HashMap::new();
        
        buchstGRI.insert("Α".nfd().collect::<String>(),"A".nfd().collect::<String>(),); 
        buchstGRI.insert("α".nfd().collect::<String>(), "a".nfd().collect::<String>());
        buchstGRI.insert("Β".nfd().collect::<String>(),"B".nfd().collect::<String>());
        buchstGRI.insert("β".nfd().collect::<String>(),"b".nfd().collect::<String>()); 
        buchstGRI.insert("Γ".nfd().collect::<String>(),"G".nfd().collect::<String>()); 
        buchstGRI.insert("γ".nfd().collect::<String>(),"g".nfd().collect::<String>()); 
        buchstGRI.insert("Δ".nfd().collect::<String>(),"D".nfd().collect::<String>()); 
        buchstGRI.insert("δ".nfd().collect::<String>(),"d".nfd().collect::<String>()); 
        buchstGRI.insert("Ε".nfd().collect::<String>(),"E".nfd().collect::<String>()); 
        buchstGRI.insert("ε".nfd().collect::<String>(),"e".nfd().collect::<String>()); 
        buchstGRI.insert("Ζ".nfd().collect::<String>(),"Z".nfd().collect::<String>()); 
        buchstGRI.insert("ζ".nfd().collect::<String>(),"z".nfd().collect::<String>()); 
        buchstGRI.insert("Η".nfd().collect::<String>(),"H".nfd().collect::<String>());
        buchstGRI.insert("η".nfd().collect::<String>(),"h".nfd().collect::<String>()); 
        buchstGRI.insert("Θ".nfd().collect::<String>(),"Th".nfd().collect::<String>()); 
        buchstGRI.insert("θ".nfd().collect::<String>(),"th".nfd().collect::<String>()); 
        buchstGRI.insert("Ι".nfd().collect::<String>(),"I".nfd().collect::<String>());
        buchstGRI.insert("ι".nfd().collect::<String>(),"i".nfd().collect::<String>());
        buchstGRI.insert("Κ".nfd().collect::<String>(), "K".nfd().collect::<String>()); 
        buchstGRI.insert("κ".nfd().collect::<String>(),"k".nfd().collect::<String>()); 
        buchstGRI.insert("Λ".nfd().collect::<String>(),"L".nfd().collect::<String>()); 
        buchstGRI.insert("λ".nfd().collect::<String>(),"l".nfd().collect::<String>()); 
        buchstGRI.insert("Μ".nfd().collect::<String>(),"M".nfd().collect::<String>()); 
        buchstGRI.insert("μ".nfd().collect::<String>(),"m".nfd().collect::<String>());
        buchstGRI.insert("Ν".nfd().collect::<String>(),"N".nfd().collect::<String>());
        buchstGRI.insert("ν".nfd().collect::<String>(),"n".nfd().collect::<String>()); 
        buchstGRI.insert("Ξ".nfd().collect::<String>(),"Xi".nfd().collect::<String>()); 
        buchstGRI.insert("ξ".nfd().collect::<String>(),"xi".nfd().collect::<String>()); 
        buchstGRI.insert("Ο".nfd().collect::<String>(),"O".nfd().collect::<String>()); 
        buchstGRI.insert("ο".nfd().collect::<String>(),"o".nfd().collect::<String>()); 
        buchstGRI.insert("Π".nfd().collect::<String>(),"P".nfd().collect::<String>()); 
        buchstGRI.insert("π".nfd().collect::<String>(),"p".nfd().collect::<String>()); 
        buchstGRI.insert("Ρ".nfd().collect::<String>(),"R".nfd().collect::<String>()); 
        buchstGRI.insert("ρ".nfd().collect::<String>(),"r".nfd().collect::<String>()); 
        buchstGRI.insert("Σ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchstGRI.insert("σ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchstGRI.insert("ς".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchstGRI.insert("Τ".nfd().collect::<String>(),"T".nfd().collect::<String>()); 
        buchstGRI.insert("τ".nfd().collect::<String>(),"t".nfd().collect::<String>()); 
        buchstGRI.insert("Υ".nfd().collect::<String>(),"U".nfd().collect::<String>()); 
        buchstGRI.insert("υ".nfd().collect::<String>(),"u".nfd().collect::<String>()); 
        buchstGRI.insert("Φ".nfd().collect::<String>(),"Ph".nfd().collect::<String>()); 
        buchstGRI.insert("φ".nfd().collect::<String>(),"ph".nfd().collect::<String>()); 
        buchstGRI.insert("Χ".nfd().collect::<String>(),"X".nfd().collect::<String>()); 
        buchstGRI.insert("χ".nfd().collect::<String>(),"x".nfd().collect::<String>()); 
        buchstGRI.insert("Ψ".nfd().collect::<String>(),"Ps".nfd().collect::<String>()); 
        buchstGRI.insert("ψ".nfd().collect::<String>(),"ps".nfd().collect::<String>()); 
        buchstGRI.insert("Ω".nfd().collect::<String>(),"O".nfd().collect::<String>()); 
        buchstGRI.insert("ω".nfd().collect::<String>(),"o".nfd().collect::<String>());
        buchstGRI
    };
    static ref LAGRIS: HashMap<String, String> = {
        let mut LAGRI = HashMap::new();
        LAGRI.insert("A".nfd().collect::<String>(),"Α".nfd().collect::<String>());
        LAGRI.insert("a".nfd().collect::<String>(),"α".nfd().collect::<String>()); 
        LAGRI.insert("B".nfd().collect::<String>(),"Β".nfd().collect::<String>()); 
        LAGRI.insert("b".nfd().collect::<String>(),"β".nfd().collect::<String>()); 
        LAGRI.insert("G".nfd().collect::<String>(),"Γ".nfd().collect::<String>()); 
        LAGRI.insert("g".nfd().collect::<String>(),"γ".nfd().collect::<String>()); 
        LAGRI.insert("D".nfd().collect::<String>(),"Δ".nfd().collect::<String>()); 
        LAGRI.insert("d".nfd().collect::<String>(),"δ".nfd().collect::<String>()); 
        LAGRI.insert("E".nfd().collect::<String>(),"Ε".nfd().collect::<String>()); 
        LAGRI.insert("e".nfd().collect::<String>(),"ε".nfd().collect::<String>()); 
        LAGRI.insert("Z".nfd().collect::<String>(),"Ζ".nfd().collect::<String>()); 
        LAGRI.insert("z".nfd().collect::<String>(),"ζ".nfd().collect::<String>()); 
        LAGRI.insert("H".nfd().collect::<String>(),"Η".nfd().collect::<String>()); 
        LAGRI.insert("h".nfd().collect::<String>(),"η".nfd().collect::<String>()); 
        LAGRI.insert("Th".nfd().collect::<String>(),"Θ".nfd().collect::<String>()); 
        LAGRI.insert("th".nfd().collect::<String>(),"θ".nfd().collect::<String>()); 
        LAGRI.insert("I".nfd().collect::<String>(),"Ι".nfd().collect::<String>()); 
        LAGRI.insert("i".nfd().collect::<String>(),"ι".nfd().collect::<String>()); 
        LAGRI.insert("K".nfd().collect::<String>(),"Κ".nfd().collect::<String>()); 
        LAGRI.insert("k".nfd().collect::<String>(),"κ".nfd().collect::<String>());
        LAGRI.insert("C".nfd().collect::<String>(),"Κ".nfd().collect::<String>()); 
        LAGRI.insert("c".nfd().collect::<String>(),"κ".nfd().collect::<String>()); 
        LAGRI.insert("Q".nfd().collect::<String>(),"Κ".nfd().collect::<String>()); 
        LAGRI.insert("q".nfd().collect::<String>(),"κ".nfd().collect::<String>()); 
        LAGRI.insert("L".nfd().collect::<String>(),"Λ".nfd().collect::<String>()); 
        LAGRI.insert("l".nfd().collect::<String>(),"λ".nfd().collect::<String>()); 
        LAGRI.insert("M".nfd().collect::<String>(),"Μ".nfd().collect::<String>()); 
        LAGRI.insert("m".nfd().collect::<String>(),"μ".nfd().collect::<String>()); 
        LAGRI.insert("N".nfd().collect::<String>(),"Ν".nfd().collect::<String>()); 
        LAGRI.insert("n".nfd().collect::<String>(),"ν".nfd().collect::<String>()); 
        LAGRI.insert("Xi".nfd().collect::<String>(),"Ξ".nfd().collect::<String>()); 
        LAGRI.insert("xi".nfd().collect::<String>(),"ξ".nfd().collect::<String>()); 
        LAGRI.insert("O".nfd().collect::<String>(),"Ο".nfd().collect::<String>()); 
        LAGRI.insert("o".nfd().collect::<String>(),"ο".nfd().collect::<String>()); 
        LAGRI.insert("P".nfd().collect::<String>(),"Π".nfd().collect::<String>()); 
        LAGRI.insert("p".nfd().collect::<String>(),"π".nfd().collect::<String>()); 
        LAGRI.insert("R".nfd().collect::<String>(),"Ρ".nfd().collect::<String>()); 
        LAGRI.insert("r".nfd().collect::<String>(),"ρ".nfd().collect::<String>()); 
        LAGRI.insert("S".nfd().collect::<String>(),"Σ".nfd().collect::<String>()); 
        LAGRI.insert("s".nfd().collect::<String>(),"σ".nfd().collect::<String>()); 
        LAGRI.insert("s".nfd().collect::<String>(),"ς".nfd().collect::<String>()); 
        LAGRI.insert("T".nfd().collect::<String>(),"Τ".nfd().collect::<String>()); 
        LAGRI.insert("t".nfd().collect::<String>(),"τ".nfd().collect::<String>()); 
        LAGRI.insert("U".nfd().collect::<String>(),"Υ".nfd().collect::<String>()); 
        LAGRI.insert("u".nfd().collect::<String>(),"υ".nfd().collect::<String>()); 
        LAGRI.insert("Ph".nfd().collect::<String>(),"Φ".nfd().collect::<String>()); 
        LAGRI.insert("ph".nfd().collect::<String>(),"φ".nfd().collect::<String>()); 
        LAGRI.insert("F".nfd().collect::<String>(),"Φ".nfd().collect::<String>()); 
        LAGRI.insert("f".nfd().collect::<String>(),"φ".nfd().collect::<String>()); 
        LAGRI.insert("V".nfd().collect::<String>(),"Φ".nfd().collect::<String>()); 
        LAGRI.insert("v".nfd().collect::<String>(),"φ".nfd().collect::<String>()); 
        LAGRI.insert("X".nfd().collect::<String>(),"Χ".nfd().collect::<String>()); 
        LAGRI.insert("x".nfd().collect::<String>(),"χ".nfd().collect::<String>()); 
        LAGRI.insert("Ps".nfd().collect::<String>(),"Ψ".nfd().collect::<String>()); 
        LAGRI.insert("ps".nfd().collect::<String>(),"ψ".nfd().collect::<String>()); 
        LAGRI.insert("O".nfd().collect::<String>(),"Ω".nfd().collect::<String>()); 
        LAGRI.insert("o".nfd().collect::<String>(),"ω".nfd().collect::<String>());
        LAGRI
    };
    pub static ref groupsS: HashMap<String, Vec<String> > = {
        let mut groups = HashMap::new();
        groups.insert("γγ".nfd().collect::<String>(),vec!["n".nfd().collect::<String>(), "g".nfd().collect::<String>()]);
        groups.insert("γκ".nfd().collect::<String>(),vec!["n".nfd().collect::<String>(), "c".nfd().collect::<String>()]); 
        groups.insert("γξ".nfd().collect::<String>(),vec!["n".nfd().collect::<String>(), "x".nfd().collect::<String>()]);
        groups.insert("γχ".nfd().collect::<String>(),vec!["n".nfd().collect::<String>(), "ch".nfd().collect::<String>()]); 
        groups.insert("ηυ".nfd().collect::<String>(),vec!["ē".nfd().collect::<String>(), "u".nfd().collect::<String>()]); //only small letters?
        groups
    };
    pub static ref behauchungS: HashMap<String, String> = { //some missing
        let mut behauchung = HashMap::new();
        behauchung.insert("῾".nfd().collect::<String>(),"h".nfd().collect::<String>()); //missing other Hauch?
        behauchung
    };
    pub static ref buchsCopticS: HashMap<String, String> = {
        let mut buchsCoptic = HashMap::new();
        buchsCoptic.insert("ϐ".nfd().collect::<String>(),"B".nfd().collect::<String>());
        buchsCoptic.insert("ϑ".nfd().collect::<String>(),"Th".nfd().collect::<String>());
        buchsCoptic.insert("ϱ".nfd().collect::<String>(),"r".nfd().collect::<String>()); 
        buchsCoptic.insert("ϰ".nfd().collect::<String>(),"k".nfd().collect::<String>()); 
        buchsCoptic.insert("ϒ".nfd().collect::<String>(),"y".nfd().collect::<String>()); 
        buchsCoptic.insert("ϕ".nfd().collect::<String>(),"ph".nfd().collect::<String>()); 
        buchsCoptic.insert("ϖ".nfd().collect::<String>(),"p".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϝ".nfd().collect::<String>(),"W".nfd().collect::<String>()); 
        buchsCoptic.insert("ϝ".nfd().collect::<String>(),"w".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϙ".nfd().collect::<String>(),"Q".nfd().collect::<String>());
        buchsCoptic.insert("ϙ".nfd().collect::<String>(),"q".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϟ".nfd().collect::<String>(),"ḳ".nfd().collect::<String>()); 
        buchsCoptic.insert("ϟ".nfd().collect::<String>(),"ḳ".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϲ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲥ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲥ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchsCoptic.insert("ϲ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchsCoptic.insert("Ͻ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchsCoptic.insert("ͻ".nfd().collect::<String>(),"s".nfd().collect::<String>());
        buchsCoptic.insert("Ϳ ".nfd().collect::<String>(),"j".nfd().collect::<String>());
        buchsCoptic.insert("ϳ".nfd().collect::<String>(),"j".nfd().collect::<String>());
        buchsCoptic.insert("Ͱ".nfd().collect::<String>(),"h".nfd().collect::<String>());
        buchsCoptic.insert("ͱ".nfd().collect::<String>(),"h".nfd().collect::<String>());
        buchsCoptic.insert("Ⲁ".nfd().collect::<String>(),"A".nfd().collect::<String>());
        buchsCoptic.insert("ⲁ".nfd().collect::<String>(),"a".nfd().collect::<String>()); 
        buchsCoptic.insert("ϴ".nfd().collect::<String>(),"t".nfd().collect::<String>());
        buchsCoptic.insert("Ⲑ".nfd().collect::<String>(),"t".nfd().collect::<String>());
        buchsCoptic.insert("ⲑ".nfd().collect::<String>(),"t".nfd().collect::<String>());
        buchsCoptic.insert("ϵ".nfd().collect::<String>(),"e".nfd().collect::<String>());
        buchsCoptic.insert("϶".nfd().collect::<String>(),"e".nfd().collect::<String>());
        buchsCoptic.insert("Ϸ".nfd().collect::<String>(),"Sh".nfd().collect::<String>());
        buchsCoptic.insert("ϸ".nfd().collect::<String>(),"sh".nfd().collect::<String>()); 
        buchsCoptic.insert("ϼ".nfd().collect::<String>(),"P".nfd().collect::<String>());
        buchsCoptic.insert("Ϡ".nfd().collect::<String>(),"S".nfd().collect::<String>());
        buchsCoptic.insert("ϡ".nfd().collect::<String>(),"S".nfd().collect::<String>());
        buchsCoptic.insert("Ⳁ".nfd().collect::<String>(),"S".nfd().collect::<String>());
        buchsCoptic.insert("ⳁ".nfd().collect::<String>(),"s".nfd().collect::<String>());
        buchsCoptic.insert("Ͳ".nfd().collect::<String>(),"Ss".nfd().collect::<String>()); 
        buchsCoptic.insert("ͳ".nfd().collect::<String>(),"ss".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϻ".nfd().collect::<String>(),"S".nfd().collect::<String>());
        buchsCoptic.insert("ϻ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϣ".nfd().collect::<String>(),"š".nfd().collect::<String>());
        buchsCoptic.insert("ϣ".nfd().collect::<String>(),"š".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϥ".nfd().collect::<String>(),"F".nfd().collect::<String>());
        buchsCoptic.insert("ϥ".nfd().collect::<String>(),"f".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϧ".nfd().collect::<String>(),"X".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⳉ".nfd().collect::<String>(),"X".nfd().collect::<String>());
        buchsCoptic.insert("ϧ".nfd().collect::<String>(),"x".nfd().collect::<String>());
        buchsCoptic.insert("ⳉ".nfd().collect::<String>(),"x".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϩ".nfd().collect::<String>(),"H".nfd().collect::<String>()); 
        buchsCoptic.insert("ϩ".nfd().collect::<String>(),"h".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϫ".nfd().collect::<String>(),"J".nfd().collect::<String>()); 
        buchsCoptic.insert("ϫ".nfd().collect::<String>(),"j".nfd().collect::<String>()); 
        buchsCoptic.insert("Ϭ".nfd().collect::<String>(),"C".nfd().collect::<String>());
        buchsCoptic.insert("ϭ".nfd().collect::<String>(),"c".nfd().collect::<String>());
        buchsCoptic.insert("Ϯ".nfd().collect::<String>(),"Di".nfd().collect::<String>());
        buchsCoptic.insert("ϯ".nfd().collect::<String>(),"di".nfd().collect::<String>()); 
        buchsCoptic.insert("Ͼ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchsCoptic.insert("Ͽ".nfd().collect::<String>(),"S".nfd().collect::<String>()); 
        buchsCoptic.insert("ͼ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchsCoptic.insert("ͽ".nfd().collect::<String>(),"s".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲃ".nfd().collect::<String>(),"B".nfd().collect::<String>());
        buchsCoptic.insert("ⲃ".nfd().collect::<String>(),"b".nfd().collect::<String>());
        buchsCoptic.insert("Ⲅ".nfd().collect::<String>(),"G".nfd().collect::<String>());
        buchsCoptic.insert("ⲅ".nfd().collect::<String>(),"g".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲇ".nfd().collect::<String>(),"D".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲇ".nfd().collect::<String>(),"d".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲉ".nfd().collect::<String>(),"E".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲉ".nfd().collect::<String>(),"e".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲋ".nfd().collect::<String>(),"St".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲋ".nfd().collect::<String>(),"st".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲍ".nfd().collect::<String>(),"Z".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲍ".nfd().collect::<String>(),"z".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲏ".nfd().collect::<String>(),"ê".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲏ".nfd().collect::<String>(),"ê".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲓ".nfd().collect::<String>(),"I".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲓ".nfd().collect::<String>(),"i".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲕ".nfd().collect::<String>(),"K".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲕ".nfd().collect::<String>(),"k".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲗ".nfd().collect::<String>(),"L".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲗ".nfd().collect::<String>(),"l".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲙ".nfd().collect::<String>(),"M".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲙ".nfd().collect::<String>(),"m".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲛ".nfd().collect::<String>(),"N".nfd().collect::<String>());
        buchsCoptic.insert("ⲛ".nfd().collect::<String>(),"n".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲝ".nfd().collect::<String>(),"ks".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲝ".nfd().collect::<String>(),"ks".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲟ	".nfd().collect::<String>(),"O".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲟ".nfd().collect::<String>(),"o".nfd().collect::<String>());
        buchsCoptic.insert("Ⲡ".nfd().collect::<String>(),"B".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲡ".nfd().collect::<String>(),"b".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲣ".nfd().collect::<String>(),"R".nfd().collect::<String>());
        buchsCoptic.insert("ⲣ".nfd().collect::<String>(),"r".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲧ".nfd().collect::<String>(),"T".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲧ".nfd().collect::<String>(),"t".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲩ".nfd().collect::<String>(),"U".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲩ".nfd().collect::<String>(),"u".nfd().collect::<String>()); 
        buchsCoptic.insert("Ⲫ".nfd().collect::<String>(),"F".nfd().collect::<String>());
        buchsCoptic.insert("ⲫ".nfd().collect::<String>(),"f".nfd().collect::<String>());
        buchsCoptic.insert("Ⲭ".nfd().collect::<String>(),"Kh".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲭ".nfd().collect::<String>(),"kh".nfd().collect::<String>());
        buchsCoptic.insert("Ⲯ".nfd().collect::<String>(),"Ps".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲯ".nfd().collect::<String>(),"ps".nfd().collect::<String>());
        buchsCoptic.insert("Ⲱ".nfd().collect::<String>(),"ô".nfd().collect::<String>()); 
        buchsCoptic.insert("ⲱ".nfd().collect::<String>(),"ô".nfd().collect::<String>()); 
        buchsCoptic.insert("Ͷ".nfd().collect::<String>(),"W".nfd().collect::<String>()); 
        buchsCoptic.insert("ͷ".nfd().collect::<String>(),"w".nfd().collect::<String>()); //
        buchsCoptic
    };
 
    //"de" Akzente richtig, oder falsch????
    pub static ref listofelusionS: HashMap<String, String> = {
        let mut listofelusion = HashMap::new();
        listofelusion.insert("δ᾽".nfd().collect::<String>(),"δὲ".nfd().collect::<String>());
        listofelusion.insert("δ'".nfd().collect::<String>(),"δὲ".nfd().collect::<String>()); 
        listofelusion.insert("ἀλλ’".nfd().collect::<String>(), "ἀλλά".nfd().collect::<String>()); 
        listofelusion.insert("ἀνθ’".nfd().collect::<String>(), "ἀντί".nfd().collect::<String>()); 
        listofelusion.insert("ἀπ’".nfd().collect::<String>(), "ἀπό".nfd().collect::<String>()); 
        listofelusion.insert("ἀφ’".nfd().collect::<String>(), "ἀπό".nfd().collect::<String>());
        listofelusion.insert("γ’".nfd().collect::<String>(), "γε".nfd().collect::<String>());
        listofelusion.insert("γένοιτ’".nfd().collect::<String>(), "γένοιτο".nfd().collect::<String>());
        listofelusion.insert("δ’".nfd().collect::<String>(), "δέ".nfd().collect::<String>());
        listofelusion.insert("δι’".nfd().collect::<String>(), "διά".nfd().collect::<String>());
        listofelusion.insert("δύναιτ’".nfd().collect::<String>(), "δύναιτο".nfd().collect::<String>());
        listofelusion.insert("εἶτ’".nfd().collect::<String>(), "εἶτα".nfd().collect::<String>());
        listofelusion.insert("ἐπ’".nfd().collect::<String>(), "ἐπί".nfd().collect::<String>());
        listofelusion.insert("ἔτ’".nfd().collect::<String>(), "ἔτι".nfd().collect::<String>());
        listofelusion.insert("ἐφ’".nfd().collect::<String>(), "ἐπί".nfd().collect::<String>());
        listofelusion.insert("ἡγοῖντ’".nfd().collect::<String>(), "ἡγοῖντο".nfd().collect::<String>());
        listofelusion.insert("ἵν’".nfd().collect::<String>(), "ἵνα".nfd().collect::<String>());
        listofelusion.insert("καθ’".nfd().collect::<String>(), "κατά".nfd().collect::<String>());
        listofelusion.insert("κατ’".nfd().collect::<String>(), "κατά".nfd().collect::<String>());
        listofelusion.insert("μ’".nfd().collect::<String>(), "με".nfd().collect::<String>());
        listofelusion.insert("μεθ’".nfd().collect::<String>(), "μετά".nfd().collect::<String>());
        listofelusion.insert("μετ’".nfd().collect::<String>(), "μετά".nfd().collect::<String>());
        listofelusion.insert("μηδ’".nfd().collect::<String>(), "μηδέ".nfd().collect::<String>());
        listofelusion.insert("μήδ’".nfd().collect::<String>(), "μηδέ".nfd().collect::<String>());
        listofelusion.insert("ὅτ’".nfd().collect::<String>(), "ὅτε".nfd().collect::<String>());
        listofelusion.insert("οὐδ’".nfd().collect::<String>(), "οὐδέ".nfd().collect::<String>());
        listofelusion.insert("πάνθ’".nfd().collect::<String>(), "πάντα".nfd().collect::<String>());
        listofelusion.insert("πάντ’".nfd().collect::<String>(), "πάντα".nfd().collect::<String>());
        listofelusion.insert("παρ’".nfd().collect::<String>(), "παρά".nfd().collect::<String>());
        listofelusion.insert("ποτ’".nfd().collect::<String>(), "ποτε".nfd().collect::<String>());
        listofelusion.insert("σ’".nfd().collect::<String>(), "σε".nfd().collect::<String>());
        listofelusion.insert("ταῦθ’".nfd().collect::<String>(), "ταῦτα".nfd().collect::<String>());
        listofelusion.insert("ταῦτ’".nfd().collect::<String>(), "ταῦτα".nfd().collect::<String>());
        listofelusion.insert("τοῦτ’".nfd().collect::<String>(), "τοῦτο".nfd().collect::<String>());
        listofelusion.insert("ὑπ’".nfd().collect::<String>(), "ὑπό".nfd().collect::<String>());
        listofelusion.insert("ὑφ’".nfd().collect::<String>(), "ὑπό".nfd().collect::<String>());
        listofelusion
    };
    pub static ref ronumS: HashMap<String, bool> = {
        let mut ronum = HashMap::new();
        ronum.insert("i".nfd().collect::<String>(),true); 
        ronum.insert("ii".nfd().collect::<String>(),true); 
        ronum.insert("iii".nfd().collect::<String>(),true);  
        ronum.insert("iiii".nfd().collect::<String>(),true);  
        ronum.insert("iv".nfd().collect::<String>(),true);  
        ronum.insert("v".nfd().collect::<String>(),true);  
        ronum.insert("vii".nfd().collect::<String>(),true);  
        ronum.insert("viii".nfd().collect::<String>(),true); 
        ronum.insert("ix".nfd().collect::<String>(),true); 
        ronum.insert("x".nfd().collect::<String>(),true); 
        ronum.insert("xi".nfd().collect::<String>(),true);  
        ronum.insert("xii".nfd().collect::<String>(),true);  
        ronum.insert("xiii".nfd().collect::<String>(),true); 
        ronum.insert("xiv".nfd().collect::<String>(),true);  
        ronum.insert("xv".nfd().collect::<String>(),true);  
        ronum.insert("xvi".nfd().collect::<String>(),true);  
        ronum.insert("xvii".nfd().collect::<String>(),true);  
        ronum.insert("xviii".nfd().collect::<String>(),true);  
        ronum.insert("xix".nfd().collect::<String>(),true);  
        ronum.insert("xx".nfd().collect::<String>(),true);  
        ronum.insert("xxi".nfd().collect::<String>(),true);  
        ronum.insert("xxii".nfd().collect::<String>(),true);  
        ronum.insert("xxiii".nfd().collect::<String>(),true);  
        ronum.insert("xxiv".nfd().collect::<String>(),true);  
        ronum.insert("xxv".nfd().collect::<String>(),true);  
        ronum.insert("xxvi".nfd().collect::<String>(),true);  
        ronum.insert("xxvii".nfd().collect::<String>(),true);  
        ronum.insert("xxviii".nfd().collect::<String>(),true);  
        ronum.insert("xxix".nfd().collect::<String>(),true);  
        ronum.insert("xxx".nfd().collect::<String>(),true);  
        ronum.insert("xxxi".nfd().collect::<String>(),true);  
        ronum.insert("xxxii".nfd().collect::<String>(),true);  
        ronum.insert("xxxiii".nfd().collect::<String>(),true);  
        ronum.insert("xxxiv".nfd().collect::<String>(),true);  
        ronum.insert("xxxv".nfd().collect::<String>(),true);  
        ronum.insert("xxxvi".nfd().collect::<String>(),true);  
        ronum.insert("xxxvii".nfd().collect::<String>(),true);  
        ronum.insert("xxxviii".nfd().collect::<String>(),true);  
        ronum.insert("xxxix".nfd().collect::<String>(),true);  
        ronum.insert("xl".nfd().collect::<String>(),true);  
        ronum.insert("xli".nfd().collect::<String>(),true);  
        ronum.insert("xlii".nfd().collect::<String>(),true);  
        ronum.insert("xliii".nfd().collect::<String>(),true);  
        ronum.insert("xliv".nfd().collect::<String>(),true);  
        ronum.insert("xlv".nfd().collect::<String>(),true);  
        ronum.insert("xlvi".nfd().collect::<String>(),true);  
        ronum.insert("xlvii".nfd().collect::<String>(),true);  
        ronum.insert("xlviii".nfd().collect::<String>(),true);  
        ronum.insert("xlix".nfd().collect::<String>(),true);  
        ronum.insert("l".nfd().collect::<String>(),true);  
        ronum.insert("li".nfd().collect::<String>(),true);  
        ronum.insert("lii".nfd().collect::<String>(),true);  
        ronum.insert("liii".nfd().collect::<String>(),true);  
        ronum.insert("liv".nfd().collect::<String>(),true);  
        ronum.insert("lv".nfd().collect::<String>(),true);  
        ronum.insert("lvi".nfd().collect::<String>(),true);  
        ronum.insert("lvii".nfd().collect::<String>(),true);  
        ronum.insert("lviii".nfd().collect::<String>(),true);  
        ronum.insert("lix".nfd().collect::<String>(),true);  
        ronum.insert("lx".nfd().collect::<String>(),true);  
        ronum.insert("lxi".nfd().collect::<String>(),true);  
        ronum.insert("lxii".nfd().collect::<String>(),true);  
        ronum.insert("lxiii".nfd().collect::<String>(),true);  
        ronum.insert("lxiv".nfd().collect::<String>(),true);  
        ronum.insert("lxv".nfd().collect::<String>(),true);  
        ronum.insert("lxvi".nfd().collect::<String>(),true);  
        ronum.insert("lxvii".nfd().collect::<String>(),true);  
        ronum.insert("lxviii".nfd().collect::<String>(),true);  
        ronum.insert("lxix".nfd().collect::<String>(),true);  
        ronum.insert("lxx".nfd().collect::<String>(),true);  
        ronum.insert("lxxi".nfd().collect::<String>(),true);  
        ronum.insert("lxxii".nfd().collect::<String>(),true);  
        ronum.insert("lxxiii".nfd().collect::<String>(),true);  
        ronum.insert("lxxiv".nfd().collect::<String>(),true);  
        ronum.insert("lxxv".nfd().collect::<String>(),true);  
        ronum.insert("lxxvi".nfd().collect::<String>(),true);  
        ronum.insert("lxxvii".nfd().collect::<String>(),true);  
        ronum.insert("lxxviii".nfd().collect::<String>(),true);  
        ronum.insert("lxxix".nfd().collect::<String>(),true);  
        ronum.insert("lxxx".nfd().collect::<String>(),true);  
        ronum.insert("lxxxi".nfd().collect::<String>(),true);  
        ronum.insert("lxxxii".nfd().collect::<String>(),true);  
        ronum.insert("lxxxiii".nfd().collect::<String>(),true);  
        ronum.insert("lxxxiv".nfd().collect::<String>(),true);  
        ronum.insert("lxxxv".nfd().collect::<String>(),true);  
        ronum.insert("lxxxvi".nfd().collect::<String>(),true);  
        ronum.insert("lxxxvii".nfd().collect::<String>(),true);  
        ronum.insert("lxxxviii".nfd().collect::<String>(),true);  
        ronum.insert("lxxxix".nfd().collect::<String>(),true);  
        ronum.insert("xc".nfd().collect::<String>(),true);  
        ronum.insert("xci".nfd().collect::<String>(),true);  
        ronum.insert("xcii".nfd().collect::<String>(),true);  
        ronum.insert("xciii".nfd().collect::<String>(),true);  
        ronum.insert("xciv".nfd().collect::<String>(),true);  
        ronum.insert("xcv".nfd().collect::<String>(),true);  
        ronum.insert("xcvi".nfd().collect::<String>(),true);  
        ronum.insert("xcvii".nfd().collect::<String>(),true);  
        ronum.insert("xcviii".nfd().collect::<String>(),true);  
        ronum.insert("xcix".nfd().collect::<String>(),true);  
        ronum.insert("c".nfd().collect::<String>(),true); 
        ronum
    };

    pub static ref grnumS: HashMap<String, bool> = {
        let mut grnum = HashMap::new(); //not perfect
        grnum.insert("α".nfd().collect::<String>(),true); 
        grnum.insert("β".nfd().collect::<String>(),true);  
        grnum.insert("γ".nfd().collect::<String>(),true);  
        grnum.insert("δ".nfd().collect::<String>(),true);  
        grnum.insert("ε".nfd().collect::<String>(),true);  
        grnum.insert("ϛ".nfd().collect::<String>(),true);  
        grnum.insert("ζ".nfd().collect::<String>(),true);  
        grnum.insert("η".nfd().collect::<String>(),true);  
        grnum.insert("θ".nfd().collect::<String>(),true);  
        grnum.insert("ι".nfd().collect::<String>(),true);  
        grnum.insert("ια".nfd().collect::<String>(),true);  
        grnum.insert("ιβ".nfd().collect::<String>(),true);  
        grnum.insert("ιγ".nfd().collect::<String>(),true);  
        grnum.insert("ιδ".nfd().collect::<String>(),true);  
        grnum.insert("ιε".nfd().collect::<String>(),true);  
        grnum.insert("ιϛ".nfd().collect::<String>(),true);  
        grnum.insert("ιζ".nfd().collect::<String>(),true);  
        grnum.insert("ιη".nfd().collect::<String>(),true);  
        grnum.insert("ιθ".nfd().collect::<String>(),true);  
        grnum.insert("κ".nfd().collect::<String>(),true);  
        grnum.insert("κα".nfd().collect::<String>(),true);  
        grnum.insert("κβ".nfd().collect::<String>(),true);  
        grnum.insert("κγ".nfd().collect::<String>(),true);  
        grnum.insert("κδ".nfd().collect::<String>(),true);  
        grnum.insert("κε".nfd().collect::<String>(),true);  
        grnum.insert("κϛ".nfd().collect::<String>(),true);  
        grnum.insert("κζ".nfd().collect::<String>(),true);  
        grnum.insert("κη".nfd().collect::<String>(),true);  
        grnum.insert("κθ".nfd().collect::<String>(),true);  
        grnum.insert("λ".nfd().collect::<String>(),true);  
        grnum.insert("λα".nfd().collect::<String>(),true);  
        grnum.insert("λβ".nfd().collect::<String>(),true);  
        grnum.insert("λγ".nfd().collect::<String>(),true);  
        grnum.insert("λδ".nfd().collect::<String>(),true);  
        grnum.insert("λε".nfd().collect::<String>(),true);  
        grnum.insert("λϛ".nfd().collect::<String>(),true);  
        grnum.insert("λζ".nfd().collect::<String>(),true);  
        grnum.insert("λη".nfd().collect::<String>(),true);  
        grnum.insert("λθ".nfd().collect::<String>(),true);  
        grnum.insert("μ".nfd().collect::<String>(),true);  
        grnum.insert("μα".nfd().collect::<String>(),true);  
        grnum.insert("μβ".nfd().collect::<String>(),true);  
        grnum.insert("μγ".nfd().collect::<String>(),true);  
        grnum.insert("μδ".nfd().collect::<String>(),true);  
        grnum.insert("με".nfd().collect::<String>(),true);  
        grnum.insert("μϛ".nfd().collect::<String>(),true);  
        grnum.insert("μζ".nfd().collect::<String>(),true);  
        grnum.insert("μη".nfd().collect::<String>(),true);  
        grnum.insert("μθ".nfd().collect::<String>(),true);  
        grnum.insert("ν".nfd().collect::<String>(),true);  
        grnum.insert("να".nfd().collect::<String>(),true);  
        grnum.insert("νβ".nfd().collect::<String>(),true);  
        grnum.insert("νγ".nfd().collect::<String>(),true);  
        grnum.insert("νδ".nfd().collect::<String>(),true);  
        grnum.insert("νε".nfd().collect::<String>(),true);  
        grnum.insert("νϛ".nfd().collect::<String>(),true);  
        grnum.insert("νζ".nfd().collect::<String>(),true);  
        grnum.insert("νη".nfd().collect::<String>(),true);  
        grnum.insert("νθ".nfd().collect::<String>(),true);  
        grnum.insert("ξ".nfd().collect::<String>(),true);  
        grnum.insert("ξα".nfd().collect::<String>(),true);  
        grnum.insert("ξβ".nfd().collect::<String>(),true);  
        grnum.insert("ξγ".nfd().collect::<String>(),true);  
        grnum.insert("ξδ".nfd().collect::<String>(),true);  
        grnum.insert("ξε".nfd().collect::<String>(),true);  
        grnum.insert("ξϛ".nfd().collect::<String>(),true);  
        grnum.insert("ξζ".nfd().collect::<String>(),true);  
        grnum.insert("ξη".nfd().collect::<String>(),true);  
        grnum.insert("ξθ".nfd().collect::<String>(),true);  
        grnum.insert("ο".nfd().collect::<String>(),true);  
        grnum.insert("οα".nfd().collect::<String>(),true);  
        grnum.insert("οβ".nfd().collect::<String>(),true);  
        grnum.insert("ογ".nfd().collect::<String>(),true);  
        grnum.insert("οδ".nfd().collect::<String>(),true);  
        grnum.insert("οε".nfd().collect::<String>(),true);  
        grnum.insert("οϛ".nfd().collect::<String>(),true);  
        grnum.insert("οζ".nfd().collect::<String>(),true);  
        grnum.insert("οη".nfd().collect::<String>(),true);  
        grnum.insert("οθ".nfd().collect::<String>(),true);  
        grnum.insert("π".nfd().collect::<String>(),true);  
        grnum.insert("πα".nfd().collect::<String>(),true);  
        grnum.insert("πβ".nfd().collect::<String>(),true);  
        grnum.insert("πγ".nfd().collect::<String>(),true);  
        grnum.insert("πδ".nfd().collect::<String>(),true);  
        grnum.insert("πε".nfd().collect::<String>(),true);  
        grnum.insert("πϛ".nfd().collect::<String>(),true);  
        grnum.insert("πζ".nfd().collect::<String>(),true);  
        grnum.insert("πη".nfd().collect::<String>(),true);  
        grnum.insert("πθ".nfd().collect::<String>(),true);  
        grnum.insert("ϟ".nfd().collect::<String>(),true);  
        grnum.insert("ϟα".nfd().collect::<String>(),true);  
        grnum.insert("ϟβ".nfd().collect::<String>(),true);  
        grnum.insert("ϟγ".nfd().collect::<String>(),true);  
        grnum.insert("ϟδ".nfd().collect::<String>(),true);  
        grnum.insert("ϟε".nfd().collect::<String>(),true);  
        grnum.insert("ϟϛ".nfd().collect::<String>(),true);  
        grnum.insert("ϟζ".nfd().collect::<String>(),true);  
        grnum.insert("ϟη".nfd().collect::<String>(),true);  
        grnum.insert("ϟθ".nfd().collect::<String>(),true);  
        grnum.insert("ρ".nfd().collect::<String>(),true); 
        grnum
    };

    pub static ref cleanhtmltags: Regex = Regex::new( "<(.*?)>" ).unwrap();
    pub static ref cleanhtmlformat1: Regex = Regex::new( "&nbsp;" ).unwrap();
    pub static ref regEbr1: Regex = Regex::new( "<br/>" ).unwrap();
    pub static ref regEbr2: Regex = Regex::new( "<br>" ).unwrap();
    pub static ref cleanNEWL: Regex = Regex::new( "\n" ).unwrap();
    pub static ref cleanRETL: Regex = Regex::new( "\r" ).unwrap();
    
    
    
    pub static ref cleanthisleer: Regex = Regex::new( "\u{00A0}" ).unwrap(); //no break space "\xa0"

    
    pub static ref cleanstrangehochpunkt: Regex = Regex::new( "let mut newastr" ).unwrap();
    
    pub static ref cleanleerpunkt: Regex = Regex::new( " \\." ).unwrap();
    pub static ref cleanleerdoppelpunkt: Regex = Regex::new( " :" ).unwrap();
    pub static ref cleanleerkoma: Regex = Regex::new( " ," ).unwrap();
    pub static ref cleanleersemik: Regex = Regex::new( " ;" ).unwrap();
    pub static ref cleanleerausrufe: Regex = Regex::new( " !" ).unwrap();
    pub static ref cleanleerfrege: Regex = Regex::new( " \\?" ).unwrap();

    //this is maybe doubbled
    pub static ref cleanpunkt: Regex = Regex::new( "\\." ).unwrap();
    pub static ref cleandoppelpunkt: Regex = Regex::new( ":" ).unwrap();
    pub static ref cleankoma: Regex = Regex::new( "," ).unwrap();
    pub static ref cleansemik: Regex = Regex::new( ";" ).unwrap();
    pub static ref cleanausrufe: Regex = Regex::new( "!" ).unwrap();
    pub static ref cleanfrege: Regex = Regex::new( "\\?" ).unwrap();
    
    pub static ref cleanthisbinde: Regex = Regex::new( "—" ).unwrap();
    pub static ref cleanklbindstrichvollbreit: Regex = Regex::new( "－" ).unwrap();
    pub static ref cleanklbindstrichkurz: Regex = Regex::new( "﹣" ).unwrap();
    pub static ref cleanklgeviert: Regex = Regex::new( "﹘" ).unwrap();
    pub static ref cleanviertelgeviert: Regex = Regex::new( " '‐" ).unwrap();
    pub static ref cleanziffbreitergeviert: Regex = Regex::new( " '‒" ).unwrap();
    pub static ref cleanhalbgeviert: Regex = Regex::new( " '–" ).unwrap();
    pub static ref cleangeviert: Regex = Regex::new( " '—" ).unwrap();
    
    pub static ref escspitzeL: Regex = Regex::new( "<" ).unwrap(); //" '<"
    pub static ref escspitzeR: Regex = Regex::new( ">" ).unwrap(); //" '>" 
    
    pub static ref behauis: Regex = Regex::new( "῾" ).unwrap();
    
    
    pub static ref dia1: Regex = Regex::new("\u{0313}").unwrap();
    pub static ref dia2: Regex = Regex::new("\u{0314}").unwrap();
    pub static ref dia3: Regex = Regex::new("\u{0300}").unwrap();
    pub static ref dia4: Regex = Regex::new("\u{0301}").unwrap(); 
    pub static ref dia5: Regex = Regex::new("\u{00B4}").unwrap();
    pub static ref dia6: Regex = Regex::new("\u{02CA}").unwrap();  
    pub static ref dia7: Regex = Regex::new("\u{02B9}").unwrap(); 
    pub static ref dia8: Regex = Regex::new("\u{0342}").unwrap();  
    pub static ref dia9: Regex = Regex::new("\u{0308}").unwrap();  
    pub static ref dia10: Regex = Regex::new("\u{0304}").unwrap(); 
    pub static ref dia11: Regex = Regex::new("\u{0306}").unwrap(); 
    pub static ref dia12: Regex = Regex::new("’").unwrap();
    pub static ref dia13: Regex = Regex::new("\'").unwrap(); 
    pub static ref dia14: Regex = Regex::new("᾽").unwrap(); 
    pub static ref dia15: Regex = Regex::new("´").unwrap(); 
    pub static ref dia16: Regex = Regex::new("‘").unwrap();

    pub static ref regJotaSub: Regex = Regex::new( "\u{0345}" ).unwrap();
    pub static ref numeringReg1: Regex = Regex::new( r"\[[0-9]+\]" ).unwrap();
    pub static ref numeringReg2: Regex = Regex::new( r"\[[M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})]+\]" ).unwrap();
    
    // precompiled regular expressions of the relevant ligatures 
    pub static ref iotasubregex: Regex = Regex::new( "\u{0345}").unwrap(); 
    pub static ref regEstigma : Regex = Regex::new( "\u{03DA}" ).unwrap(); 
    pub static ref regEstigmakl: Regex = Regex::new( "\u{03DB}" ).unwrap();
    pub static ref regEomikonyplsi: Regex = Regex::new( "ȣ" ).unwrap();
    pub static ref regEomikonyplsiK: Regex = Regex::new( "Ȣ" ).unwrap();
    pub static ref regEUk: Regex = Regex::new( "Ꙋ" ).unwrap();
    pub static ref regEuk: Regex = Regex::new( "ꙋ" ).unwrap();
    pub static ref regEkai: Regex = Regex::new( "ϗ" ).unwrap();
    pub static ref regEKai: Regex = Regex::new( "Ϗ" ).unwrap();
    pub static ref regEl1: Regex = Regex::new( "\u{0223}" ).unwrap();
    pub static ref regEl2: Regex = Regex::new( "\u{0222}" ).unwrap();
    pub static ref unterPu: Regex = Regex::new("\u{0323}" ).unwrap();
    
    pub static ref regEdoppelP: Regex = Regex::new( ":" ).unwrap();
    pub static ref regEeinfahP: Regex = Regex::new( "\\." ).unwrap();
    pub static ref regEkomma: Regex = Regex::new( "," ).unwrap();
    pub static ref regEsemiK: Regex = Regex::new( ";" ).unwrap();
    pub static ref regEhochP: Regex = Regex::new( "·" ).unwrap();
    pub static ref regEausr: Regex = Regex::new( "!" ).unwrap();
    pub static ref regEfarge: Regex = Regex::new( "\\?" ).unwrap();
    pub static ref regEan1: Regex = Regex::new( "“" ).unwrap();
    pub static ref regEan5: Regex = Regex::new( "„" ).unwrap();
    pub static ref regEan2: Regex = Regex::new( "”" ).unwrap();
    pub static ref regEan3: Regex = Regex::new( "\"" ).unwrap();
    pub static ref regEan4: Regex = Regex::new( "\'" ).unwrap();
    pub static ref regEan6: Regex = Regex::new( r"(\s*)[∶|⋮|·|⁙|;]+(\s*)" ).unwrap();

    pub static ref regU1: Regex = Regex::new( r"†" ).unwrap();
    pub static ref regU2: Regex = Regex::new( r"\\*" ).unwrap();
    pub static ref regU3: Regex = Regex::new( r"⋖" ).unwrap();
    pub static ref regU4: Regex = Regex::new( r"\#" ).unwrap();
    
    pub static ref regEtailingsig: Regex = Regex::new(r"ς" ).unwrap();

    pub static ref regEkla1: Regex = Regex::new( r"\(" ).unwrap();
    pub static ref regEkla2: Regex = Regex::new( r"\)" ).unwrap();
    pub static ref regEkla3: Regex = Regex::new( r"\{" ).unwrap();
    pub static ref regEkla4: Regex = Regex::new( r"\}" ).unwrap();
    pub static ref regEkla5: Regex = Regex::new( r"\[" ).unwrap();
    pub static ref regEkla6: Regex = Regex::new( r"\]" ).unwrap();
    pub static ref regEkla7: Regex = Regex::new( r"<" ).unwrap();
    pub static ref regEkla8: Regex = Regex::new( r">" ).unwrap();
    pub static ref regEkla9: Regex = Regex::new( r"⌈" ).unwrap();
    pub static ref regEkla10: Regex = Regex::new( r"⌉" ).unwrap();
    pub static ref regEkla11: Regex = Regex::new( r"‹" ).unwrap();//Anführungszeichen
    pub static ref regEkla12: Regex = Regex::new( r"›" ).unwrap();//Anführungszeichen
    pub static ref regEkla13: Regex = Regex::new( r"«" ).unwrap();//Anführungszeichen
    pub static ref regEkla14: Regex = Regex::new( r"»" ).unwrap();//Anführungszeichen!!! move it !!!
    pub static ref regEkla15: Regex = Regex::new( r"⟦" ).unwrap();
    pub static ref regEkla16: Regex = Regex::new( r"⟧" ).unwrap();
    pub static ref regEkla17: Regex = Regex::new( "\u{3008}" ).unwrap();
    pub static ref regEkla18: Regex = Regex::new("\u{3009}" ).unwrap();
    pub static ref regEkla19: Regex = Regex::new("\u{2329}" ).unwrap();
    pub static ref regEkla20: Regex = Regex::new("\u{232A}" ).unwrap();
    pub static ref regEkla21: Regex = Regex::new("\u{27E8}" ).unwrap();
    pub static ref regEkla22: Regex = Regex::new("\u{27E9}" ).unwrap();
    
    pub static ref regEuv: Regex = Regex::new( "u" ).unwrap();
    pub static ref spai1: Regex = Regex::new("\u{2002}" ).unwrap();//enspacing
    pub static ref spai2: Regex = Regex::new("\u{2000}" ).unwrap();//enquad
    }
    
    
    //**************************************************
    // Section 0000
    // helper
    //**************************************************
    pub fn isnumber( maybe: &String ) -> bool {
        //do romannumbers
        match maybe.trim().parse::<i32>() {
            Err(_e) => false, 
            Ok(_n) => return true,
        };
        //check arrays
        if ronumS.contains_key( maybe ) {
            return true;     
        } else if grnumS.contains_key( maybe ) {
            return true;
        }
        //no matches - uh
        return false;
    }

    //******************************************************************************
    // Section 000
    // basic UNICODE NORMAL FORM / TRANSLITERATION / Typ
    //******************************************************************************
    pub fn normarrayk( aarray: &HashMap<String, String> ) -> HashMap<String, String> {
	    let mut replacearray: HashMap<String, String> =  HashMap::new();
	    for (p,d) in aarray {
		    replacearray.insert( disambiguDIAkritika( &p.nfd().collect::<String>() ), disambiguDIAkritika(d) ); 
	    }
	    return replacearray;
    }

    pub fn normarrayksiguv( aarray: &HashMap<String, String> ) -> HashMap<String, String> {
	    let mut replacearray: HashMap<String, String> =  HashMap::new();
	    for (p,d) in aarray {
		    replacearray.insert( sigmaistgleich( &deluv( &disambiguDIAkritika( &p.nfd().collect::<String>() ))), disambiguDIAkritika(d) );
	    }
	    return replacearray;
    }

    pub fn normarrayval( aarray: &mut HashMap<String, String> ){
        for d in aarray.values_mut( ){
            *d = disambiguDIAkritika( &d.nfd().collect::<String>() );
        }
    }

    pub fn normarrayvalsiguv( aarray: &mut HashMap<String, String> ){ // by reference ????
        for d in aarray.values_mut( ){
            *d = sigmaistgleich( &deluv( &disambiguDIAkritika( &d.nfd().collect::<String>() ))) ;
        }
    }

    pub fn normatextNFD( text: &String ) -> String {
        return text.nfd().collect::<String>();
    }

    pub fn normatextNFC( text: &String ) -> String {
        return text.nfc().collect::<String>();
    }

    pub fn normatextNFKD( text: &String ) -> String {
        return text.nfkd().collect::<String>();
    }

    pub fn normatextNFKC( text: &String ) -> String {
        return text.nfkc().collect::<String>();
    }

    pub fn sameallspacing( astr: &String ) -> String {
        let mut newastr = spai1.replace_all( astr, " " ).to_string( );
        newastr = spai2.replace_all( &newastr, " " ).to_string( );
        return newastr;
    }

    pub fn disambiguDIAkritika( astr: &String ) -> String {
        let mut newastr = astr.split( "\u{0027}" ).collect::<Vec<&str>>().join( "\u{2019}" ); //typogra korrektes postroph;
        newastr = newastr.split( "'" ).collect::<Vec<&str>>().join( "\u{2019}" );
        newastr = newastr.split( "\u{1FBD}" ).collect::<Vec<&str>>().join( "\u{2019}" );  
        return newastr;  
    }

    pub fn disambiguadashes( astring: &String ) -> String {
        let mut newastr = cleangeviert.replace_all( astring, "-" ).to_string( );
        newastr = cleanhalbgeviert.replace_all( &newastr, "-" ).to_string( );
        newastr = cleanziffbreitergeviert.replace_all( &newastr, "-" ).to_string( );
        newastr = cleanviertelgeviert.replace_all( &newastr, "-" ).to_string( );
        newastr = cleanklgeviert.replace_all( &newastr, "-" ).to_string( );
        newastr = cleanklbindstrichkurz.replace_all( &newastr, "-" ).to_string( );
        newastr = cleanklbindstrichvollbreit.replace_all( &newastr, "-" ).to_string( );
        return newastr;
    }

    pub fn disambiguasatzei( astring: &String ) -> String {
        let mut newastr = cleanstrangehochpunkt.replace_all( astring,"·").to_string( );
        newastr = cleanleerpunkt.replace_all( &newastr, ".").to_string( );
        newastr = cleanleerdoppelpunkt.replace_all( &newastr, ":").to_string( );
        newastr = cleanleerkoma.replace_all( &newastr, ",").to_string( );
        newastr = cleanleersemik.replace_all( &newastr, ";").to_string( );
        newastr = cleanleerausrufe.replace_all( &newastr, "!").to_string( );
        newastr = cleanleerfrege.replace_all( &newastr, "?").to_string( );
        return newastr;
    }

    pub fn ExtractDiafromBuchst( buchst: &String ) -> Vec<String> { //that will extract buchs with dia and other signes as brackets
        //println!("vor {}\n\n", buchst ); 
        let toitter = normatextNFKD( buchst );
        //println!("nach {}\n\n", toitter ); 
        let mut b: Vec<String> = vec![];
        let mut d: Vec<String> = vec![];
        for  t in toitter.chars( ){
            let co =  t.to_lowercase().to_string();
            //println!("-- Value {}\n\n", textdecomp::avalue);
            if buchstGRIS.contains_key( &co ) || buchsCopticS.contains_key( &co ) || textdecomp::buchstLATS.contains_key( &co ) { 
                b.push( t.to_string() );
            } else {
                d.push( t.to_string() );
            }
        }
        return vec![d.join(""), b.join("")];
    }

    pub fn ExtractDiafromBuchstText( atext: &String ) -> Vec<Vec<String>> {
        let mut t = vec![];
        let spli = atext.split( " " );
        for i in spli {
            let j = i.to_string();
            t.push( ExtractDiafromBuchst( &j ) );
        }
        return t;
    }

    pub fn replaceBehauchung( adiakstring: &String ) -> String {
        let mut bb = String::from("");
        if behauis.is_match( adiakstring ){
             bb = "h".to_string( ) + &behauis.replace( adiakstring, "" ).to_string();
        } 
        return bb;
    }
    

    pub fn Expandelision( aword: &String ) -> &String {
        //if word in listofelusion
        if listofelusionS.contains_key( aword ) {
            return listofelusionS.get( aword ).unwrap();
        } else {
            return aword;
        }
    }

    pub fn ExpandelisionText( atext: &String ) -> String {
        let mut t = String::from("");
        let wds = atext.split( " " );
        for w in wds {
            //println!("-- Word {} Result {}\n\n", w, Expandelision(  &w.to_string() ) );
            t.push_str( &" ".to_string( ) );
            t.push_str( Expandelision(  &w.to_string() ) );
        }
        return t;
    }

    pub fn TranslitLatinGreekLetters( astring: &String ) -> String {
        //
        let normia = normatextNFD( astring ).trim( ).to_string( );
        let ohnedia = delligaturen( &normia );
        let wordlevel = ohnedia.split( " " );
        let mut greekenized: Vec<String> = vec![ ];
        for w in wordlevel {
            let wewa = w.to_string( );
            let mut buchstlevel: Vec<&str> = Expandelision( &wewa ).split( "" ).collect();
            buchstlevel.remove(0);
            let lele = buchstlevel.len( );
            //println!("-- lele {}\n", lele );
            
            let mut perword: Vec<String> = vec![];
            let mut extractedida2:  String = String::from("");
            let mut extracteBUCHST2: String = String::from("");
            for b in 1..lele {
                let A = buchstlevel[ b-1 ].to_string( );
                let B = buchstlevel[ b ].to_string( );
                let zwischenerg1 = ExtractDiafromBuchst( &A );
                let zwischenerg2 = ExtractDiafromBuchst( &B );
                //console.log(zwischenerg1, zwischenerg2);
                
                
                let extractedida1 = &zwischenerg1[0];
                    extractedida2 = zwischenerg2[0].clone( );
                let extracteBUCHST1 = &zwischenerg1[1];
                    extracteBUCHST2 = zwischenerg2[1].clone( );
                println!("-- o1 {}, o2 {}, e1 {}, d1 {}, e2 {}, d2 {}\n", A, B, extracteBUCHST1, extractedida1, extracteBUCHST2, extractedida2);
                
                //console.log("o1", buchstlevel[ b-1 ], "o2", buchstlevel[ b ], "e1", extracteBUCHST1, "d1", extractedida1, "e2", extracteBUCHST2, "d2", extractedida2);
                let test1 = String::from("") + extracteBUCHST1 + &extracteBUCHST2;
                if LAGRIS.contains_key( &test1 ) && extracteBUCHST1 != "" && extracteBUCHST2 != "" {
                    perword.push( LAGRIS[ &test1 ].clone( ) + extractedida1 + &extractedida2 );
                } else {
                    if LAGRIS.contains_key( extracteBUCHST1 ) {
                        perword.push( LAGRIS[ extracteBUCHST1 ].clone( ) + extractedida1 );
                    } else {
                        perword.push( buchstlevel[ b-1 ].to_string( ) );
                    }
                }
                
            }
            if LAGRIS.contains_key( &extracteBUCHST2 ) {
                
                perword.push( LAGRIS[ &extracteBUCHST2 ].clone( ) + &extractedida2 );
            } else {
                perword.push( buchstlevel[ lele-1 ].to_string( ) );
            }
            greekenized.push( perword.join( "" ) );
            
        }
        //return astring;
        return greekenized.join( " " );
    }

    pub fn TraslitAncientGreekLatin( astring: &String ) -> String {
        //
        let normia = normatextNFD( astring ).trim( ).to_string( );
        let iotagemacht = iotasubiotoad( &normia );
        let ohnedia = delligaturen( &iotagemacht );
        //hier eventuell noch NFC, let wordlevel = delligaturen(  astring.trim().normalize( "NFD" ) ).normalize( "NFC" ) ).split(" ");
        let wordlevel = ohnedia.split( " " );

        //de !!!
        let mut romanized: Vec<String> = vec![];
        for w in wordlevel {
            let wewa = w.to_string( );
            let mut buchstlevel: Vec<&str> = Expandelision( &wewa ).split( "" ).collect();
            buchstlevel.remove(0);
            let mut grouped: Vec<String> = vec![];
            let mut notlastdone: bool = true;
            let mut extractedida2:  String = String::from("");
            let mut extracteBUCHST2:  String = String::from("");
            let lele = buchstlevel.len( );
            for b in 1..lele {

                if buchstlevel[ b-1 ] == "" {
                    continue;
                }

                let A = buchstlevel[ b-1 ].to_string( );
                let B = buchstlevel[ b ].to_string( );
                let zwischenerg1 = ExtractDiafromBuchst( &A );
                let zwischenerg2 = ExtractDiafromBuchst( &B );
                let extractedida1 = &zwischenerg1[0];
                    extractedida2 = zwischenerg2[0].clone();
                let extracteBUCHST1 = &zwischenerg1[1];
                    extracteBUCHST2 = zwischenerg2[1].clone();

                let test1 = String::from("") + extracteBUCHST1 + &extracteBUCHST2;
                let extractedida1ohneb = replaceBehauchung( extractedida1 );

                if groupsS.contains_key( &test1 ) && extractedida2.contains( "¨" ) { //wenn kein trema über dem zweiten buchstaben - diaresis keine Zusammenziehung (synresis)
                    let gou = groupsS[ &test1 ].clone();
                    let g = replaceBehauchung( &extractedida2 );
                    let topush = String::from("") + &gou[0] + &extractedida1ohneb + &gou[1] + &g; //NFC???
                    grouped.push( topush );
                    buchstlevel[ b ] = "";//delet alread in group and revistible
                    notlastdone = false;
                } else {
                    if buchstGRIS.contains_key( extracteBUCHST1 ) {
                        let gooo = buchstGRIS[ extracteBUCHST1 ].clone();
                        let topush2 = String::from("") + &gooo + &extractedida1ohneb;
                        grouped.push( topush2 ); //NFC ?;
                    } else {
                        if buchsCopticS.contains_key( extracteBUCHST1 ) {
                            let coooo = buchsCopticS[extracteBUCHST1].clone();
                            let topush3 = String::from("") + &coooo + &extractedida1ohneb;
                            grouped.push( topush3 );//NFC
                        } else {
                            //realy not - leave IT
                            grouped.push( buchstlevel[ b-1 ].to_string( ) );
                        }
                    }
                    notlastdone = true;
                }
            }
            if notlastdone {
                let g = replaceBehauchung( &extractedida2 );
                if buchstGRIS.contains_key( &extracteBUCHST2 ) {
                        let ggg = buchstGRIS[ &extracteBUCHST2 ].clone();
                        let topush4 = ggg + &g;
                        grouped.push( topush4 );//NFC?
                } else {
                    if buchsCopticS.contains_key( &extracteBUCHST2 ) {
                        let ttt = buchsCopticS[ &extracteBUCHST2 ].clone();
                        let topush5 = ttt + &g;
                        grouped.push( topush5 );//NFC ?
                    } else {
                        //realy not - leave IT
                        grouped.push( buchstlevel[ lele - 1 ].to_string( ) );
                    }
                }
            }
            romanized.push( grouped.join( "" ) );
        }
        return romanized.join( " " );  
    }

    //******************************************************************************
    // Section 00 
    // basic cleaning and string conversion via regexp 
    //******************************************************************************
    
    pub fn spitzeklammernHTML( astring: &String ) -> String {
        let mut newastr = escspitzeL.replace_all( astring, "&lt;" ).to_string( );
        newastr = escspitzeR.replace_all( &newastr, "&gt;" ).to_string( );
        return newastr;
    }

    pub fn basClean( astring: &String ) -> String {
        let mut newastr = cleanNEWL.replace_all( astring, " <br/>" ).to_string( );
        newastr = cleanRETL.replace_all( &newastr, " <br/>").to_string( );
        
        newastr = cleanthisbinde.replace_all( &newastr," — ").to_string( );
        newastr = cleanthisleer.replace_all( &newastr, " ").to_string( );
        newastr = disambiguasatzei( &newastr );
        newastr = disambiguadashes( &newastr );


        // remove hyphens
        let ws: Vec<&str> = newastr.split( " " ).collect();
        let ca = Trennstricheraus( &ws );
        return ca.join( " " );
        //let wss = ohnesatzzeichen( &ws );
        
        //return wss.join( " " );
    }

    pub fn delsatzei( astr: &String ) -> String {
        let mut newastr = cleanstrangehochpunkt.replace_all( astr,"").to_string( );
        newastr = cleanpunkt.replace_all( &newastr, "").to_string( );
        newastr = cleandoppelpunkt.replace_all( &newastr, "").to_string( );
        newastr = cleankoma.replace_all( &newastr, "").to_string( );
        newastr = cleansemik.replace_all( &newastr, "").to_string( );
        newastr = cleanausrufe.replace_all( &newastr, "").to_string( );
        newastr = cleanfrege.replace_all( &newastr, "").to_string( );
        return newastr;
    }
    
    pub fn tennsatzei( astr: &String ) -> String {
        let mut newastr = cleanstrangehochpunkt.replace_all( astr,"  ·").to_string( );
        newastr = cleanpunkt.replace_all( &newastr, " .").to_string( );
        newastr = cleandoppelpunkt.replace_all( &newastr, " :").to_string( );
        newastr = cleankoma.replace_all( &newastr, " ,").to_string( );
        newastr = cleansemik.replace_all( &newastr, " ;").to_string( );
        newastr = cleanausrufe.replace_all( &newastr, " !").to_string( );
        newastr = cleanfrege.replace_all( &newastr, " ?").to_string( );
        return newastr;
    }

    pub fn ohnesatzzeichen( wliste: &Vec<&str> ) -> Vec<String> {
        let mut nwliste: Vec<String> = vec![];
        let lele = wliste.len( );
		for w in 0..lele {
            let content = wliste[ w ].to_string();
			nwliste.push( delsatzei( &content ) );
		}
        return nwliste;
    }

    //usage: replaceWordsfromarray( ["in", "cum", "et", "a", "ut"], " ", stringggg )
    /*pub fn replaceWordsfromarray( arr: &Vec<String>, replacement: str, strstr: &String ){
        let mut newastr = String::from( strstr );
        let lele = arr.len( );
        for a in 0..lele {
            newastr = &newastr.replace( arr[a], replacement );
        }
        return newastr;
    }*/

    //******************************************************************************
    // Section 0
    // word leve conversions: 
    // alpha privativum
    // alpha copulativum
    //******************************************************************************

    pub fn AlphaPrivativumCopulativum( aword: &String ) -> String { //just works on NFC and NFKC
        let newaword = String::from( aword );
        let umcod = normatextNFC( &newaword );
        let mut origletters: Vec<&str> = umcod.split( "" ).collect( );
        origletters.remove(0);
        if !notprivalphaS.contains_key( aword ) {
            let deleded = delall( aword );
            let mut buchs: Vec<&str> = deleded.split( "" ).collect( );
            buchs.remove(0);
            //println!("nochmal alle {:?} {:?}\n\n", buchs, origletters );
            if buchs[0].contains( "α" ) { //erste Buchstabe alpha
                if textdecomp::vokaleGRIS.contains_key( buchs[1] ) { // zweiter ein Vokal
                    let extracted = ExtractDiafromBuchst( &origletters[1].to_string() );
                    let b2dia = &extracted[ 0 ];
                    //println!("before right {:?}\n\n", extracted );
                    if b2dia.contains( "\u{0308}" ) { //zweiter Buchstabe mit Trema, erste Buchstabe mit spiritus lenis
                        let a = origletters[0].to_string( ); 
                        origletters.remove(0);
                        let r = origletters.join( "" );
                        //println!("IN right {}\n\n", r ); 
                        let tempstring = String::from("") + &a +" "+ &r;
                        return normatextNFD( &tempstring );
                    } else { //
                        return newaword;
                    }
                } else {
                    return newaword;
                }
            } else {
                return newaword;
            }
        } else {
            return newaword;
        }   
    }

    pub fn AlphaPrivativumCopulativumText( atext: &String ) -> String {
        let mut t = String::from( "" );
        let spli = atext.split( " " );
        for l in spli {
            let ll = l.to_string();
            let conc = String::from(" ")+&AlphaPrivativumCopulativum( &ll );
            t += &conc;
        }
        return t;
    }

    //******************************************************************************
    // Section 1 
    // unicode related comparing and norming, handling of diacritics
    //******************************************************************************

    //function takes string, repl jota subscriptum with jota adscriptum
    pub fn iotasubiotoad( aword: &String ) -> String {
     	return iotasubregex.replace_all( aword, "ι" ).to_string( );
    }

    //function replaces diacritica
    pub fn ohnediakritW( aword: &String ) -> String {
        let mut mstristrastru = dia1.replace_all( aword, "" ).to_string( );
        mstristrastru = dia2.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia3.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia4.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia5.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia6.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia7.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia8.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia9.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia10.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia11.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia12.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia13.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia14.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia15.replace_all( &mstristrastru, "" ).to_string( );
        mstristrastru = dia16.replace_all( &mstristrastru, "" ).to_string( );
        return mstristrastru;
    }

    //firs letter capitalized, all other letters small
    pub fn capitali( astring: &String ) -> String {
        let umcod = normatextNFC( astring );
        let mut alsbuchs: Vec<&str> = umcod.split( "" ).collect( );
        alsbuchs.remove( 0 );
        let firstletter = alsbuchs[0].to_string( );
        alsbuchs.remove( 0 );
        let rest = alsbuchs.join( "" );
        return firstletter.to_uppercase() + &rest;
    }

    //function takes a string replaces diacritica and iotasubscriptum with iota adscriptum
    pub fn nodiakinword( aword: &String ) -> String {
        return iotasubiotoad( &ohnediakritW( aword ) );
    }

    //******************************************************************************
    // Section 2: deleting things that could be not same in two texts
    //******************************************************************************
    // function takes string and converts tailing sigma to inline sigma (greek lang)
    pub fn sigmaistgleich( text: &String ) -> String {
        return regEtailingsig.replace_all( text, "σ" ).to_string( );
    }

    // function takes string and replaces u by v, used in classical latin texts
    pub fn deluv( text: &String ) -> String {
        return regEuv.replace_all( text, "v" ).to_string( );
    }

    pub fn delligaturen( text: &String ) -> String {
        let mut mstristrastru = regEstigma.replace_all( text, "στ" ).to_string( );
        mstristrastru = regEstigmakl.replace_all( &mstristrastru, "στ").to_string( );
        mstristrastru = regEUk.replace_all( &mstristrastru, "Υκ").to_string( );
        mstristrastru = regEuk.replace_all( &mstristrastru, "υκ").to_string( );
        mstristrastru = regEomikonyplsi.replace_all( &mstristrastru, "ου").to_string( );
        mstristrastru = regEomikonyplsiK.replace_all( &mstristrastru, "ου").to_string( );
        mstristrastru = regEkai.replace_all( &mstristrastru, "καὶ").to_string( );
        mstristrastru = regEKai.replace_all( &mstristrastru, "Καὶ").to_string( );
        mstristrastru = regEl1.replace_all( &mstristrastru, "\u{039F}\u{03C5}" ).to_string( );
        mstristrastru = regEl2.replace_all( &mstristrastru, "\u{03BF}\u{03C5}" ).to_string( );
        return mstristrastru;
    }


    //function take a string and deletes diacritical signes, ligatures, remaining interpunction, line breaks, capital letters to small ones, equalizes sigma at the end of greek words, and removes brakets
    pub fn delall( text: &String ) -> String {
        let mut ninu = String::from("");
        if doUVlatin { // convert u to v in classical latin text
            ninu = deluv( &delklammern( &sigmaistgleich( &delgrkl( &delumbrbine( &delligaturen( &delinterp( &delmakup( &delnumbering( &delunknown( &deldiak(text)))))))))));
        } else {
            ninu = delklammern( &sigmaistgleich( &delgrkl( &delumbrbine( &delligaturen( &delinterp( &delmakup( &delnumbering( &delunknown( &deldiak(text))))))))));
        }
        return ninu;
    }

    //run this before the klammern deletion
    pub fn delnumbering( text: &String ) -> String {
        let mut ninu = numeringReg1.replace_all( text, "" ).to_string( );
        ninu = numeringReg2.replace_all( &ninu, "" ).to_string( );
        return ninu;
    }

    //function takes string and splits it into words, than normalizes each word, joins the string again
    pub fn deldiak( text: &String ) -> String {
        let spt: Vec<&str> = text.split( " " ).collect( ); //seperate words
        let lele = spt.len();
        let mut retvec: Vec<String> = vec![]; 
        for wi in 0..lele {
            let tempstring = spt[ wi ].to_string();
            retvec.push( nodiakinword( &tempstring ) );
        }
        return retvec.join( " " );
    }    

    //function takes a string and replaces interpunction
    pub fn delinterp( text: &String ) -> String {
        let mut ninu = regEdoppelP.replace_all( text, "").to_string( );
        ninu = regEeinfahP.replace_all( &ninu, "").to_string( );
        ninu = regEkomma.replace_all( &ninu, "").to_string( );
        ninu = regEsemiK.replace_all( &ninu, "").to_string( );
        ninu = regEhochP.replace_all( &ninu, "").to_string( );
        ninu = regEausr.replace_all( &ninu, "").to_string( );
        ninu = regEfarge.replace_all( &ninu, "").to_string( );
        ninu = regEan1.replace_all( &ninu, "").to_string( );
        ninu = regEan2.replace_all( &ninu, "").to_string( );
        ninu = regEan3.replace_all( &ninu, "").to_string( );
        ninu = regEan4.replace_all( &ninu, "").to_string( );
        ninu = regEan5.replace_all( &ninu, "").to_string( );
        ninu = regEan6.replace_all( &ninu, "").to_string( );
        return ninu;
    }

    //function takes a string and replaces some unknown signs
    pub fn delunknown( text: &String ) -> String {
        let mut ninu = regU1.replace_all( text, "").to_string( );
        ninu = regU2.replace_all( &ninu, "").to_string( );
        ninu = regU3.replace_all( &ninu, "").to_string( );
        ninu = regU4.replace_all( &ninu, "").to_string( );
        return ninu;
    }

    //function takes string and replace html line breakes
    pub fn delumbrbine( text: &String ) -> String {
        let mut ninu = regEbr1.replace_all( text, "").to_string( );
        ninu = regEbr2.replace_all( &ninu, "").to_string( );
        return ninu;
    }

    pub fn umbrtospace( text: &String ) -> String {
        let mut ninu = cleanNEWL.replace_all( text, " ").to_string( );
        ninu = cleanRETL.replace_all( &ninu, " ").to_string( );
        ninu = regEbr1.replace_all( &ninu, " ").to_string( );
        ninu = regEbr2.replace_all( &ninu, " ").to_string( );
        return ninu;
    }

    //more to come
    pub fn delmakup( text: &String ) -> String {
        let mut ninu = cleanhtmltags.replace_all( text, "").to_string( );
        ninu = cleanhtmlformat1.replace_all( &ninu, "").to_string( );
        return ninu;
    }

    pub fn makuptoleer( text: &String ) -> String { 
        let mut ninu = cleanhtmltags.replace_all( text, " ").to_string( );
        ninu = cleanhtmlformat1.replace_all( &ninu, " ").to_string( );
        return ninu;
    }

    // ...
    pub fn delgrkl( text: &String ) -> String {
        return text.to_lowercase( );
    }

    // function take sstring and replaces the brakets -- do not run this before the Klammersystem fkt
    pub fn delklammern( text: &String ) -> String {
        let mut ninu = regEkla1.replace_all( text, "").to_string( );
        ninu = regEkla2.replace_all( &ninu, "").to_string( );
        ninu = regEkla3.replace_all( &ninu, "").to_string( );
        ninu = regEkla4.replace_all( &ninu,"").to_string( );
        ninu = regEkla5.replace_all( &ninu,"").to_string( );
        ninu = regEkla6.replace_all( &ninu,"").to_string( );
        ninu = regEkla7.replace_all( &ninu,"").to_string( );
        ninu = regEkla8.replace_all( &ninu,"").to_string( );
        ninu = regEkla9.replace_all( &ninu,"").to_string( );
        ninu = regEkla10.replace_all( &ninu,"").to_string( );
        ninu = regEkla11.replace_all( &ninu,"").to_string( );
        ninu = regEkla12.replace_all( &ninu,"").to_string( );
        ninu = regEkla13.replace_all( &ninu,"").to_string( );
        ninu = regEkla14.replace_all( &ninu,"").to_string( );
        ninu = regEkla15.replace_all( &ninu,"").to_string( );
        ninu = regEkla16.replace_all( &ninu,"").to_string( );
        ninu = regEkla17.replace_all( &ninu,"").to_string( );
        ninu = regEkla18.replace_all( &ninu,"").to_string( );
        ninu = regEkla19.replace_all( &ninu,"").to_string( );
        ninu = regEkla20.replace_all( &ninu,"").to_string( );
        ninu = regEkla21.replace_all( &ninu,"").to_string( );
        ninu = regEkla22.replace_all( &ninu,"").to_string( );
        return ninu;
    }

    pub fn deledklammern( text: &String ) -> String {
        let mut ninu = regEkla1.replace_all( text, "").to_string( );
        ninu = regEkla2.replace_all( &ninu, "").to_string( );
        ninu = regEkla3.replace_all( &ninu, "").to_string( );
        ninu = regEkla4.replace_all( &ninu,"").to_string( );
        ninu = regEkla5.replace_all( &ninu,"").to_string( );
        ninu = regEkla6.replace_all( &ninu,"").to_string( );
        ninu = regEkla9.replace_all( &ninu,"").to_string( );
        ninu = regEkla10.replace_all( &ninu,"").to_string( );
        ninu = regEkla11.replace_all( &ninu,"").to_string( );
        ninu = regEkla12.replace_all( &ninu,"").to_string( );
        ninu = regEkla13.replace_all( &ninu,"").to_string( );
        ninu = regEkla14.replace_all( &ninu,"").to_string( );
        ninu = regEkla15.replace_all( &ninu,"").to_string( );
        ninu = regEkla16.replace_all( &ninu,"").to_string( );
        ninu = regEkla17.replace_all( &ninu,"").to_string( );
        ninu = regEkla18.replace_all( &ninu,"").to_string( );
        ninu = regEkla19.replace_all( &ninu,"").to_string( );
        ninu = regEkla20.replace_all( &ninu,"").to_string( );
        ninu = regEkla21.replace_all( &ninu,"").to_string( );
        ninu = regEkla22.replace_all( &ninu,"").to_string( );
        return ninu;
    }


    //some bundels
    pub fn Trennstricheraus( wliste: &Vec<&str> ) -> Vec<String> {
	    let mut ersterteil: String = String::from( "" );
	    let mut neueWLISTE: Vec<String> = vec![];
        let lele = wliste.len();
	    for  w in 0..lele {
            //println!("w {} ersterteil {}\n\n", wliste[ w ], ersterteil); 
		    if ersterteil == "" {
			    if wliste[ w ].contains( "-" ) {
				    let eUNDz: Vec<&str> = wliste[ w ].split( "-" ).collect();
				    if eUNDz[1] != "" {
					    let zweiohnenewline: Vec<&str> = eUNDz[1].split( "\n" ).collect( );
                        
			     		neueWLISTE.push( String::from("")+eUNDz[0]+zweiohnenewline[ zweiohnenewline.len()-1 ] );
				    } else {
                        let mut i = w+1;
                        if i == lele {
                            i = w
                        }
                        if wliste[ i ].contains( "\n" ) {
					        ersterteil = eUNDz[0].to_string( );
                        } else {
                            neueWLISTE.push( wliste[ w ].to_string() );
                        }
				    }
			    } else { //nix - normales wort
                    let tempstring = wliste[ w ].to_string( );
				    neueWLISTE.push( tempstring );
			    }
		    } else { // es gab eine Trennung und die ging über zwei Listenzellen
			    if wliste[ w ].contains( "[" ) && !wliste[ w ].contains( "]" ) {
				    let zweiteralsliste: Vec<&str> = wliste[ w ].split( "\n" ).collect( );
				    neueWLISTE.push( ersterteil.clone() + &zweiteralsliste[ zweiteralsliste.len()-1 ] );
				    ersterteil.clear();
			    } else { //klammern behandeln
					     //wenn ich hier kein push auf der neune Wortliste mache, dann lösche ich damit die geklammerten sachen

				    if wliste[ w ].contains( "[" ) && wliste[ w ].contains( "]" ) { //klammern in einem Wort
					    let zweiteralsliste: Vec<&str> = wliste[ w ].split( "]" ).collect( );
                        let mut m: Vec<&str> = zweiteralsliste[zweiteralsliste.len()-1].split( "" ).collect( );
                        m.remove(0);
                        m.remove(0);
                        let inser = m.join( "" );
					    neueWLISTE.push( ersterteil.clone() + &inser );
				    } else if wliste[ w ].contains( "[" ) {
					    let zweiteralsliste: Vec<&str> = wliste[ w ].split( "[" ).collect( );
					    neueWLISTE.push( ersterteil.clone() + &zweiteralsliste.join("") );
				    } else { //nur schließende Klammer
					    let zweiteralsliste: Vec<&str> = wliste[ w ].split( "]" ).collect( );
					    neueWLISTE.push( ersterteil.clone() + &zweiteralsliste[ zweiteralsliste.len()-1 ] );
				    }
			    }
		    }
	    }
	    return neueWLISTE;
    }

    pub fn UmbruchzuLeerzeichen( atext: &String ) -> String {
	    return atext.split("\n").collect::<Vec<&str>>().join( " " );
    }

    pub fn Interpunktiongetrennt( wliste: &Vec<&str> ) -> Vec<String> {
        let mut neuewliste: Vec<String> = vec![];
        for w in wliste {
            let tempstring = w.to_string( );
            let getrenntalsstring = tennsatzei( &tempstring );
            let getalsarr:Vec<&str> = getrenntalsstring.split( " " ).collect( );
            let lele = getalsarr.len( );
            if lele > 1 {
                for i in 0..lele {
                    neuewliste.push( getalsarr[ i ].to_string() );
                }
            } else {
                neuewliste.push( tempstring );
            }
        }
	    return neuewliste;
    }

    pub fn iotasubiotoadL( wliste: &Vec<&str> ) -> Vec<String> {
        let mut neuewliste: Vec<String> = vec![];
        let lele = wliste.len();
	    for w in 0..lele {
            neuewliste.push( iotasubiotoad( &wliste[ w ].to_string() ) );
		   
	    }
        return neuewliste;
    }

    //function to use with greek text maybe
    pub fn GRvorbereitungT( dtext: &String ) -> Vec<String> {
        let a = delnumbering( dtext );
        let b = normatextNFD( &a );
        let c = b.to_lowercase( );
        let d = disambiguDIAkritika( &c );
        let e = d.split( " " ).collect::<Vec<&str>>( );
        let f = Trennstricheraus( &e );
        let g = f.join( " " );
        let h = UmbruchzuLeerzeichen( &g );
        let i = h.split( " " ).collect::<Vec<&str>>( );
        let j = Interpunktiongetrennt( &i );
        //Klammernbehandeln( diewo );
	      
	    return j;
    } 

    
    pub fn delUnterpunkt( text: &String ) -> String {
        return unterPu.replace_all( text, "" ).to_string( );
    }
} //end textnorm


