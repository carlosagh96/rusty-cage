use std::io;

//Rusty calc
//Interactive CLI calculator
//Not finished

fn rline(get_only_an_int:bool)->String
{
	let mut line=String::new();
	let mut valid:bool=false;
	loop
	{
		io::stdin().read_line(&mut line).expect("pfff");

		if line.trim().len()>0
		{
			if get_only_an_int
			{
				valid={
					let thy_bin:u32=match line.trim().parse::<u32>()
					{
						Ok(m)=>
						{
							1
						},
						Err(_)=>
						{
							0
						}
					};
					let vvv={
						if thy_bin==1{true}else{false}
					};
					vvv
				};
				if valid
				{
					break;
				}
				else
				{
					line.clear();
				}
			}
			else
			{
				break;
			}
		}
	};
	line
}

fn choose_data_qtty()->u32
{
	let qtty={
		let raw_str=rline(true);
		let raw:u32=match raw_str.trim().parse()
		{
			Ok(m)=>{m},
			Err(_)=>{0}
		};
		raw
	};
	qtty
}

fn data_input()->(bool,f32)
{
	//let mut indata_raw=String::new();
	let mut failed=false;
	let indata_raw=rline(false);
	let indata={
		let candidate:f32=match indata_raw.trim().parse()
		{
			Ok(num)=>{num},
			Err(_)=>{failed=true;0.0}
		};
		candidate
	};
	let result=(failed,indata);
	result
}

fn main()
{
	let mut get_out=false;
	let mut data_qtty:u32=0;
	println!("Calculadora interactiva\n=======================\nEscrito por カルロサグ\nhttps://t.me/CarlosAGH\n2022-03-24\n");
	//Get Ammount of data
	loop
	{
		println!("Inserte la cantidad de datos");
		data_qtty=choose_data_qtty();
		if data_qtty>1 || data_qtty==0
		{
			if data_qtty==0
			{
				get_out=true;
			}
			break;
		}
	}

	//Get Out
	if get_out
	{
		println!("Aweno, temecuida...");
		return;
	}

	let mut data_bank:Vec<(bool,f32)>=vec![];
	{
		println!("Introduzca {} datos",data_qtty);
		let mut idx=0;
		while idx<data_qtty
		{
			println!("Dato {}",idx+1);
			data_bank.push(data_input());
			idx=idx+1;
		}
	}
	let the_total={
		let mut total=0.0;
		for pair in data_bank
		{
			if pair.0
			{
				println!("pfff");
			}
			else
			{
				total=total+pair.1;
			}
		}
		total
	};
	println!("Total {}",the_total);
}
