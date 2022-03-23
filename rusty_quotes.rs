const DECORATION:&str="================================================================================";

/*
// Quotes as structs
struct Quote
{
	text:String,
	auth:String
}
impl Quote
{
	fn new(a:&str,t:&str)->Quote
	{
		Quote
		{
			text:t.to_string(),
			auth:a.to_string()
		}
	}
	fn show(&self)->String
	{
		format!("\"{}\"\n\n{}",self.text,self.auth)
	}
}
*/

fn main()
{
	let thy_args:Vec<String>=std::env::args().collect();
	if !(thy_args.len()==2)
	{
		return;
	}
	let index:&str=&thy_args[1];
	let index:usize=match index.trim().parse()
	{
		Ok(num)=>num,
		Err(_)=>return
	};

	// Quotes as structs

	// Quotes as structs. Template
	// quotes.push(Quote::new("author","text"));

	/*
	let mut quotes:Vec<Quote>=vec![];

	quotes.push(Quote::new("Anónimo","El que nace para culo, la patada le cae del cielo"));
	quotes.push(Quote::new("Dicho cubano","DPEPDPE"));
	quotes.push(Quote::new("Dicho cubano","Cuando el mal es de cagar, no valen guayabas verdes"));	

	quotes.push(Quote::new("Dante Alighieri","Los lugares más oscuros del infierno están reservados para aquellos que mantienen su neutralidad en tiempos de crisis moral"));
	quotes.push(Quote::new("Huey Newton","Un pueblo desarmado es un esclavo o está sujeto a la esclavitud en cualquier momento"));
	quotes.push(Quote::new("Vladimir Ilyich Ulyanov","Los hechos son testarudos"));
	quotes.push(Quote::new("Vladimir Putin","Quien no eche de menos la unión soviética, no tiene corazón. Quien quiere que vuelva, no tiene cerebro"));
	*/

	//Quotes as tuples

	let mut quotes:Vec<(&str,&str)>=vec![];

	//Quotes as tuples. Template
	//quotes.push(("Author","Text"));

	quotes.push(("Anónimo","El que nace para culo, la patada le cae del cielo"));
	quotes.push(("Dicho cubano","DPEPDPE (De pinga el país de pinga este)"));
	quotes.push(("Dicho cubano","Cuando el mal es de cagar, no valen guayabas verdes"));

	//Get quote vector length
	if index>=quotes.len()
	{
		return;
	}

	//Quotes as structs
	//println!("{}",quotes[index].show());

	//Quotes as tuples
	let the_quote=&quotes[index];
	println!("{}\n\"{}\"\n\n{}\n{}",DECORATION,&the_quote.1,&the_quote.0,DECORATION);
}
