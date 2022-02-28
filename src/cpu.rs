struct Cpu {
	memory: [Mem; 255]
}

enum Mem {
	Str([char; 8]),
	Int(i64),
	Float(f64)
}