use std::convert::TryInto;
use std::env;

//Rusty Quotes

//May contain offensive content. Look away

//Constants
const STR_DEC:&str="================================================================================";
const STR_LB:&str="\n";
const STR_SPACE:&str=" ";

fn main()
{
	let thy_args:Vec<String>=env::args().collect();

	// Show error and get out
	if thy_args.len()>2
	{
		println!("ERROR: Número de argumentos no válido");
		return;
	}

	//Show help
	if thy_args.len()==1
	{
		let ppp=&thy_args[0];
		println!("\nFrases\n======\n\nEscrito por カルロサグ\nhttps://t.me/CarlosAGH\n2022-03-24\n\nUsos\n\n$ {0} total\n└─> Obtener el total de frases\n\n$ {0} N\n└─> Mostrar la frase número N\n",ppp);
		return;
	}

	let mut return_quote=true;
	let index_raw:&str=&thy_args[1];
	let index:usize=match index_raw.trim().parse()
	{
		Ok(num)=>
		{
			num
		},
		Err(_)=>
		{
			return_quote=false;
			0
		}
	};

	if !return_quote
	{
		let show_total=if index_raw=="total"{true}else{false};
		if !show_total
		{
			println!("ERROR: Argumento no válido");
			return;
		}
	}

	let mut quotes:Vec<(&str,&str)>=vec![];

	//Anonimous
	quotes.push(("Anónimo","No confíes en nadie, recuerda que el diablo fue un angel"));
	quotes.push(("Anónimo","La gente te amará por lo que eres y otros te odiarán por la misma razón"));
	quotes.push(("Anónimo","Es mejor estar loco y ser feliz que ser normal y amargo"));
	quotes.push(("Anónimo","Procura nunca salir de tu casa enojado con aquellos que amas. La vida puede cambiar en un segundo, tal vez nunca te despidas"));
	quotes.push(("Anónimo","Hoy son errores, mañana serán experiencias"));
	quotes.push(("Anónimo","Nunca subestimes a una persona callada. Las cosas importantes se planean en silencio"));
	quotes.push(("Anónimo","La madurez está en la mente y no en la edad"));
	quotes.push(("Anónimo","Ser buena persona no significa dejarse pisotear por los demás"));
	quotes.push(("Anónimo","Vive sin miedo, nacimos para morir"));
	quotes.push(("Anónimo","El tiempo no te hará olvidar, te hará madurar y entender mejor las cosas"));
	quotes.push(("Anónimo","El peor error en la vida es dar demasiada importancia a personas que no valen la pena. Tu tiempo es oro"));
	quotes.push(("Anónimo","No te preocupes por lo que digan de tí porque ni Dios ha logrado caerle bien a todo el mundo"));

	//Cuban sayings
	quotes.push(("Dicho cubano","DPEPDPE\n(De pinga el país de pinga este)"));
	quotes.push(("Dicho cubano","Cuando el mal es de cagar, no valen guayabas verdes"));
	quotes.push(("Dicho cubano","El que nace para culo, la patada le cae del cielo"));

	//Computer Science & Programming
	quotes.push(("Alan Kay","Mucho del software hoy en día se parece a una pirámide egipcia: con millones de ladrillos apilados uno encima del otro, sin integridad estructural y hecho por pura fuerza bruta y miles de esclavos"));
	quotes.push(("Bill Gates","Medir el progreso de la programación por líneas de código es como medir el progreso en la construcción de aviones por el peso"));
	quotes.push(("Bill Gates","El éxito es un pésimo profesor. Seduce a la gente inteligente y la lleva a pensar que no puede perder"));
	quotes.push(("Bill Gates","Solo hay un truco para el software, y es usar un componente de software que ya haya sido escrito"));
	quotes.push(("Bjarne Stroustrup","Sólo hay dos tipos de lenguajes: aquellos de los que la gente se queja y aquellos que nadie usa"));
	quotes.push(("Bjarne Stroustrup","Siempre deseé que mi computadora fuera tan fácil de usar como mi teléfono. Mi deseo se ha hecho realidad: ya no sé usar mi teléfono"));
	quotes.push(("Brian Kernighan","Depurar es dos veces más difícil que escribir el código en primer lugar. Entonces si escribes el código tan astutamente como sea posible, no eres (por definición) tan listo como para depurarlo"));
	quotes.push(("Charles Antony Richard Hoare","Hay dos formas de diseñar software: la primera es hacerlo tan simple que sea obvio que no tiene deficiencias y la segunda es hacerlo tan complicado que no sea obvio que tiene deficiencias. La primera forma es mucho más difícil"));
	quotes.push(("Martin Fowler","Cualquier tonto puede escribir código que un ordenador entiende. Los buenos programadores escriben código que los humanos pueden entender"));

	//Engineers & Entrepenourship
	quotes.push(("Elon Musk","Me gustaria morir en Marte, pero no en el impacto"));
	quotes.push(("Henry Ford","Los que renuncian son más numerosos que los que fracasan"));
	quotes.push(("Henry Ford","Nada es difícil si se divide en pequeñas partes"));
	quotes.push(("Henry Ford","Pensar es el trabajo más difícil que existe, tal vez por eso son unos pocos los que lo tienen"));
	quotes.push(("Henry Ford","El fracaso es una gran oportunidad para empezar otra vez con más inteligencia"));
	quotes.push(("Henry Ford","El verdadero progreso es el que pone la tecnología al alcance de todos"));
	quotes.push(("Henry Ford","Los obstáculos son esas cosas espantosas que ves cuando apartas los ojos de tu"));
	quotes.push(("Henry Ford","El mejor automovilista es aquel que conduce con imaginación. Imagina que su familia va con él en el auto"));

	//Scientists
	quotes.push(("Albert Einstein","La vida es muy peligrosa, no por las personas que hacen el mal, sino por las que se sientan a ver lo que pasa"));
	quotes.push(("Albert Einstein","No podemos resolver problemas usando el mismo tipo de pensamiento que usamos cuando los creamos"));
	quotes.push(("Albert Einstein","Text"));
	quotes.push(("Isaac Newton","Lo que sabemos es una gota de agua; lo que ignoramos es el océano"));
	quotes.push(("Isaac Newton","Los hombres construimos demasiados muros y no suficientes puentes"));
	quotes.push(("Isaac Newton","Si he hecho descubrimientos invaluables ha sido más por tener paciencia que cualquier otro talento"));
	quotes.push(("John von Neumann","Si la gente no piensa que las matemáticas son simples, es sólo porque no se dan cuenta de lo complicada que es la vida"));
	quotes.push(("John von Neumann","No tiene ningún sentido ser preciso cuando ni siquiera sabes de lo que estás hablando"));
	quotes.push(("John von Neumann","En matemáticas uno no entiende las cosas, se acostumbra a ellas"));
	quotes.push(("John von Neumann","La verdad es demasiado complicada como para permitir nada más allá de meras aproximaciones"));
	quotes.push(("John von Neumann","Cualquiera que considere métodos aritméticos para producir dígitos aleatorios"));
	quotes.push(("John von Neumann","Es más fácil viajar en un avión, incluso pilotarlo, que entender por qué puede volar"));
	quotes.push(("Stephen Hawking","Dios no sólo juega a los dados. Incluso a veces los lanza donde no pueden ser vistos"));
	quotes.push(("Stephen Hawking","El peor enemigo del conocimiento no es la ignorancia, es la ilusión del conocimiento"));
	quotes.push(("Stephen Hawking","La inteligencia es la habilidad de adaptarse a los cambios"));
	quotes.push(("Stephen Hawking","La vida sería trágica si no fuera graciosa"));
	quotes.push(("Stephen Hawking","Nada puede existir para siempre"));
	quotes.push(("Stephen Hawking","Para explicar el cosmos no necesitamos a Dios, pero la ciencia no podrá decir por qué se creó"));
	quotes.push(("Stephen Hawking","Se han concedido muchos premios Nobel por demostrar que el universo no es tan simple como se podía haber pensado"));
	quotes.push(("Stephen Hawking","Si los extraterrestres nos visitaran, ocurriría lo mismo que cuando Cristóbal Colón desembarcó en América y nada salió bien para los nativos americanos"));
	quotes.push(("Stephen Hawking","La ciencia no deja mucho espacio ni para milagros ni para Dios"));

	//Politics (Liberalism, Nationalism, Anti-Marxism, Anti-Comunism and Anti-Socialism)
	quotes.push(("Abraham Lincoln","El gobierno del pueblo, por el pueblo, para el pueblo no debe desaparecer de la tierra"));
	quotes.push(("Abraham Lincoln","Asegúrate de poner tus pies en el lugar correcto, luego mantente firme"));
	quotes.push(("Abraham Lincoln","Aquellos que buscan el mal en las personas seguro lo encontrarán"));
	quotes.push(("Abraham Lincoln","Prefiero ser un don nadie pequeño a ser un malvado alguien"));
	quotes.push(("Abraham Lincoln","Aquellos que le niegan la libertad a otros, no la merecen"));
	quotes.push(("Abraham Lincoln","Enseñen a los niños para que no sea necesario enseñar a los adultos"));
	quotes.push(("Abraham Lincoln","Todos los humanos nacen iguales, sin embargo, es la última vez que lo somos"));
	quotes.push(("Abraham Lincoln","Una persona es como un árbol y su reputación es como su sombra. La sombra es lo que pensamos de esa persona, el árbol es su verdadero ser"));
	quotes.push(("Karl Popper","El marxismo murió de marxismo"));
	quotes.push(("Karl Popper","Quien sea incapaz de hablar claro debe callar hasta poder hacerlo"));
	quotes.push(("Ludwig Von Mises","Si la Historia pudiese enseñarnos algo, sería que la propiedad privada está inextricablemente unida con la civilización"));
	quotes.push(("Ludwig Von Mises","Ninguna persona o grupo de personas deben ser retenidos contra su voluntad en una asociación política en la cual no quieren participar"));
	quotes.push(("Ludwig Von Mises","No es de extrañar que todos los que alguna vez han tenido algo nuevo que ofrecer a la humanidad no han tenido nunca nada bueno que decir sobre el Estado o sus leyes"));
	quotes.push(("Ludwig Von Mises","Tan pronto como abandonamos el principio que el gobierno no debe interferir en ningún asunto relacionado con el estilo de vida del individuo, terminamos regulando y restringiendo a este último hasta los más mínimos detalles"));
	quotes.push(("Ludwig Von Mises","Todo impuesto específico, así como todo el sistema de impuestos de una nación, se invalida a sí mismo por encima de una cierta tasa de impuestos"));
	quotes.push(("Ludwig Von Mises","Durante miles de años, en todas partes del mundo habitado, innumerables sacrificios han sido realizados a la quimera de los precios justos y razonables"));
	quotes.push(("Ludwig Von Mises","La crítica científica no tiene fin más noble que el de destruir creencias"));
	quotes.push(("Ludwig Von Mises","La Ciencia no nos da certeza absoluta y definitiva. Solamente nos da seguridad dentro de los límites de nuestras habilidades mentales y del estado en el que esté el pensamiento científico"));
	quotes.push(("Ludwig Von Mises","El criterio de la verdad es que funciona incluso cuando nadie está preparado para reconocerlo"));
	quotes.push(("Ludwig Von Mises","La primera cosa que necesita un genio es respirar aire libre"));
	quotes.push(("Ludwig Von Mises","Los negocios son un medio, el único medio, de incrementar la cantidad de bienes disponible para preservar la vida y hacerla más agradable"));
	quotes.push(("Milton Friedman","Colón no buscó una nueva ruta a las Indias en respuesta a una directiva elegida mayoritariamente"));
	quotes.push(("Milton Friedman","Es una suerte que la sociedad libre sea más productiva, porque de no haberlo sido jamás se habría tolerado. El prejuicio en su contra es tan poderoso que es necesario tener una ventaja de cinco a uno para vencerlo"));
	quotes.push(("Milton Friedman","El poder centralizado no se vuelve inofensivo por las buenas intenciones de quienes lo crearon"));
	quotes.push(("Milton Friedman","Mucha gente quiere que el gobierno proteja a los consumidores. Un problema mucho más urgente es proteger a los consumidores del gobierno"));
	quotes.push(("Milton Friedman","Nada es tan permanente como un programa temporal del gobierno"));
	quotes.push(("Milton Friedman","Los grandes avances de la civilización, ya sean en arquitectura o pintura, en ciencia o literatura, jamás han venido de un gobierno central"));

	//Misc
	quotes.push(("Dante Alighieri","Los lugares más oscuros del infierno están reservados para aquellos que mantienen su neutralidad en tiempos de crisis moral"));
	quotes.push(("Huey Newton","Un pueblo desarmado es un esclavo o está sujeto a la esclavitud en cualquier momento está, por supuesto, en pecado mortal"));
	quotes.push(("Jesucristo","No he venido a traer la paz, sino la espada"));
	quotes.push(("Vladimir Ilyich Ulyanov","Los hechos son testarudos"));
	quotes.push(("Vladimir Putin","Quien no eche de menos la unión soviética, no tiene corazón. Quien quiere que vuelva, no tiene cerebro"));

	//Get the total quotes
	if index_raw=="total"
	{
		println!("{}",quotes.len());
		return;
	}

	//Get quote vector length
	if index>=quotes.len()
	{
		println!("ERROR: Se sale del límite");
		return;
	}
	//Obtaining quote as String from &str
	let the_quote=&quotes[index];
	let text_raw={
		let quote_index=&the_quote.1;
		let evolved=quote_index.to_string();
		evolved
	};

	//Create new text
	let text_new={
		let mut text_final=String::from("");
		let mut lines_base:Vec<String>=vec![];
		if text_raw.contains(STR_LB)
		{
			for line in text_raw.split(STR_LB)
			{
				lines_base.push(line.to_string());
			}
		}
		else
		{
			lines_base.push(text_raw);
		}
		let mut line_break_lpos:u32=0;
		let mut text_x=String::from("");
		for line_x in lines_base.iter()
		{
			for line_y in line_x.split_whitespace()
			{
				let curr_len:u32={
					let mut text_y_tmp=String::from("");
					text_y_tmp.push_str(&text_x);
					text_y_tmp.push_str(line_y);
					text_y_tmp.push_str(STR_SPACE);
					text_y_tmp.len().try_into().unwrap()
				};
				if curr_len-line_break_lpos>80
				{
					let line_y_len:u32=line_y.len().try_into().unwrap();
					line_break_lpos=curr_len-line_y_len;
					text_x.push_str(STR_LB);
					text_x.push_str(line_y);
					text_x.push_str(STR_SPACE);
				}
				else
				{
					text_x.push_str(line_y);
					text_x.push_str(STR_SPACE);
				}
			}
			line_break_lpos=0;
			text_final.push_str(&text_x);
			text_final.push_str(STR_LB);
			text_x.clear();
		}
		text_final
	};
	println!("{}\n\"{}\"\n\n{}\n{}",STR_DEC,&text_new.trim(),&the_quote.0,STR_DEC);
}
