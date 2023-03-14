use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut domains = HashMap::new();
    // there were a lot of odd .cf and .ga domains with no content
    // the others catch additional unsuitable domains
    let tlds_to_remove = [
        ".bc.ca",
        ".cf",
        ".fl.us",
        ".ga",
        ".gle",
        ".gob.es",
        ".go.jp",
        ".goog",
        ".google",
        ".gov",
        ".gov.bs",
        ".gov.co",
        ".gov.gr",
        ".gov.in",
        ".gov.lb",
        ".gov.uk",
        ".gv.at",
        ".lg.jp",
        ".mb.ca",
        ".mn.us",
        ".mod.uk",
        ".nhs.uk",
        ".on.ca",
        ".pa.us",
        ".tx.us",
        ".va.us",
        ".vu.nl",
        ".youtube",
        ".yp.to",
    ];
    // used to detect domains with a dot in the tld
    let multipart_tlds = [
        "ac.be",
        "ac.jp",
        "ac.uk",
        "ac.za",
        "ad.jp",
        "co.il",
        "co.in",
        "co.jp",
        "com.au",
        "co.nz",
        "co.uk",
        "in.ua",
        "km.ua",
        "ne.jp",
        "net.au",
        "org.tw",
        "org.uk",
        "or.jp",
        "pp.ua",
        "sapporo.jp",
        "tokyo.jp",
        "wz.cz",
        "zt.ua",
    ];
    // remove non-primary domains like "google-analytics.com"
    let words_to_remove = [
        "akamai",
        "amazon",
        "buffer",
        "ebay",
        "etsy",
        "facebook",
        "flickr",
        "github",
        "google",
        "microsoft",
        "netflix",
        "oreilly",
        "paypal",
        "quora",
        "slack",
        "springer",
        "squarespace",
        "twitter",
        "vimeo",
        "wix",
        "youtube",
    ];
    // prevent unrelated domains from getting removed because it includes a words_to_remove entry
    let words_to_remove_exceptions = [
        "flickriver", // flickr
        "kiwix", // wix
        "thepiratebay", // ebay
        "whats-on-netflix", // netflix
    ];
    // generics, shorteners, redirects, domain hacks (telegra.ph), and other unsuitable domains
    // https://en.wikipedia.org/wiki/Domain_hack
    // telegra.ph is unsuitable because no mapping preserves its branding
    let domains_to_remove = [
        "1drv.ms",
        "aculo.us",
        "adf.ly",
        "adobe.ly",
        "aka.ms",
        "amzn.to",
        "binged.it",
        "bit.do",
        "bit.ly",
        "blo.gs",
        "bonus.ly",
        "brabantn.ws",
        "buff.ly",
        "camera.it",
        "chng.it",
        "cl.ly",
        "ct.de",
        "cutt.ly",
        "dai.ly",
        "db.tt",
        "dngr.us",
        "fed.us",
        "finderfee.me",
        "flic.kr",
        "gc.ca",
        "generalassemb.ly",
        "genial.ly",
        "geni.us",
        "ge.tt",
        "gg.gg",
        "ghc.us",
        "goo.gl",
        "gov.au",
        "gov.br",
        "gov.cn",
        "gov.ge",
        "gov.hk",
        "gov.ie",
        "gov.il",
        "gov.it",
        "gov.je",
        "gov.mt",
        "gov.ph",
        "gov.pl",
        "gov.scot",
        "gov.sg",
        "gov.si",
        "gov.tr",
        "gov.tw",
        "gov.ua",
        "gov.uk",
        "gov.wales",
        "gov.za",
        "gowoa.me",
        "gstatic.com",
        "hubs.ly",
        "icio.us",
        "igg.me",
        "inoa.fi",
        "instagr.am",
        "is.gd",
        "itun.es",
        "kck.st",
        "list.ly",
        "mailchi.mp",
        "narrative.ly",
        "nautil.us",
        "news.com.au",
        "nhs.uk",
        "nimb.ws",
        "nyti.ms",
        "ow.ly", //
        "page.link",
        "parliament.uk",
        "pca.st",
        "pin.it",
        "plot.ly",
        "prn.to",
        "rb.gy",
        "realestate.com.au",
        "rebrand.ly",
        "redd.it",
        "repl.it",
        "royal.uk",
        "rurl.me",
        "sickboy.wz.cz",
        "smarturl.it",
        "spoti.fi",
        "telegra.ph",
        "tim.ly",
        "trkn.us",
        "vigl.us",
        "wa.me",
        "wapo.st",
        "wdt.me",
        "web.dev",
        "we.tl",
        "wp.me",
        "youtu.be",
        "yt.be",
    ];
    
    let mut unique_slds: HashSet<String> = HashSet::new();
    
    // https://www.domcop.com/openpagerank/what-is-openpagerank
    // download the top10milliondomains.csv file to the directory with Cargo.toml in it
    // https://www.domcop.com/files/top/top10milliondomains.csv.zip
    // "Data last updated: 25th Sep 2022"
    
    process_input_file(
        "top10milliondomains.csv",
        &tlds_to_remove,
        &mut unique_slds,
        &mut domains,
        &multipart_tlds,
        &words_to_remove,
        &words_to_remove_exceptions,
        &domains_to_remove
    )?;
    
    fn process_line(
        line: &str,
        tlds_to_remove: &[&str],
        unique_slds: &mut HashSet<String>,
        domains: &mut HashMap<String, Vec<String>>,
        multipart_tlds: &[&str],
        words_to_remove: &[&str],
        words_to_remove_exceptions: &[&str],
        domains_to_remove: &[&str]
    ) {
        process_valid_line(
            line,
            tlds_to_remove,
            unique_slds,
            domains,
            &multipart_tlds,
            words_to_remove,
            words_to_remove_exceptions,
            domains_to_remove
        );
    }

    fn remove_domains_with_tld(domain: &str, tlds_to_remove: &[&str]) -> bool {
        tlds_to_remove.iter().any(|&tld| domain.ends_with(tld))
    }

    fn process_domain(
        domain: &str,
        unique_slds: &mut HashSet<String>,
        domains: &mut HashMap<String, Vec<String>>,
        rank: &str,
        multipart_tlds: &[&str],
        words_to_remove: &[&str],
        words_to_remove_exceptions: &[&str],
        domains_to_remove: &[&str]
    ) {
        if domains_to_remove.contains(&domain) {
            return;
        }
        
        if let Some(_suffix) = domains_to_remove.iter()
            .find(|&suffix| domain.ends_with(&format!(".{}", suffix)))
        {
            return;
        }
        
        let domain_parts: Vec<_> = domain.split('.').collect();
        
        let tld_parts_len = if let Some(&tld) = multipart_tlds
            .iter().find(|&tld| domain.ends_with(tld))
        {
            tld.split('.').count()
        } else {
            1
        };
        
        let tld_parts = if tld_parts_len >= domain_parts.len() {
            return;
        } else {
            domain_parts[domain_parts.len() - tld_parts_len..].to_vec()
        };
        
        let tld = tld_parts.join(".");
        let sld = domain_parts[domain_parts.len() - tld_parts_len - 1];

        // 1 and 2 character domains are not reserved
        // 1 character domains are mostly short redirects like "a.co", "t.co", and "w.org"
        // 2 character domains are removed to be able to include additional longer domains
        if sld.len() < 3 || sld.len() > 16 {
            return;
        }
        
        let should_remove = words_to_remove.iter().any(|&word| {
            sld.contains(word) && sld != word && !words_to_remove_exceptions.contains(&sld)
        });
        
        if should_remove {
            return;
        }

        if !unique_slds.insert(sld.to_owned()) {
            return;
        }

        let main_domain = format!("{}.{}", sld, tld);
        
        domains.entry(main_domain.to_owned()).or_insert_with(|| vec![]).push(rank.to_owned());
    }


    fn process_valid_line(
        line: &str,
        tlds_to_remove: &[&str],
        unique_slds: &mut HashSet<String>,
        domains: &mut HashMap<String, Vec<String>>,
        multipart_tlds: &[&str],
        words_to_remove: &[&str],
        words_to_remove_exceptions: &[&str],
        domains_to_remove: &[&str]
    ) {
        let fields: Vec<_> = line.split(',').map(|field| field.trim_matches('"')).collect();
        
        if fields.len() == 3 {
            let rank = fields[0];
            let domain = fields[1];

            if remove_domains_with_tld(domain, tlds_to_remove) {
                return;
            }

            process_domain(
                domain,
                unique_slds,
                domains,
                rank,
                &multipart_tlds,
                words_to_remove,
                words_to_remove_exceptions,
                domains_to_remove
            );
        }
    }

    fn process_input_file(
        filename: &str,
        tlds_to_remove: &[&str],
        unique_slds: &mut HashSet<String>,
        domains: &mut HashMap<String, Vec<String>>,
        multipart_tlds: &[&str],
        words_to_remove: &[&str],
        words_to_remove_exceptions: &[&str],
        domains_to_remove: &[&str]
    ) -> Result<(), Box<dyn Error>> {
        let input_file = File::open(filename)?;
        let input_reader = BufReader::new(input_file);
        let mut processed_rows = 0;

        for line_result in input_reader.lines() {
            let line = line_result?;
            if processed_rows == 0 {
                processed_rows += 1;
                continue; // skip header row
            }
            if domains.len() == 10000 {
                break; // stop processing after 10k domains
            }
            process_line(
                &line,
                tlds_to_remove,
                unique_slds,
                domains,
                &multipart_tlds,
                words_to_remove,
                words_to_remove_exceptions,
                domains_to_remove
            );
            processed_rows += 1;
        }
        
        Ok(())
    }

    let mut sorted_domains: Vec<_> = domains.iter().collect();
    sorted_domains.sort_by_key(|(_, ranks)| ranks[0].parse::<u32>().unwrap());

    let mut unique_domains = HashSet::new();
    let output_file = File::create("10k-reserved-domains.csv")?;
    let mut output_writer = BufWriter::new(output_file);

    output_writer.write_all(format!("\"Rank\",\"Domain\"\n").as_bytes())?;

    let mut current_rank = 0;

    for (domain, ranks) in sorted_domains.iter() {
        if unique_domains.insert(domain.to_owned()) {
            let rank = ranks[0].parse::<u32>().unwrap();
            if rank != current_rank {
                current_rank = rank;
                output_writer.write_all(
                    format!("\"{}\",\"{}\"\n", current_rank, domain).as_bytes()
                )?;
            } else {
                output_writer.write_all(format!("\"\",\"{}\"\n", domain).as_bytes())?;
            }
        }
    }

    output_writer.flush()?;
    println!("10k-reserved-domains.csv has been written and should have sha256 hash 05d7257769904da71a864601b37e0e5522b7ac45e26259191201f71c236ac5ae");

    Ok(())
}
