if ARGV.length != 2
    puts "We need exactly two arguments"
    exit
end

file = "target/release/tacho"
os=ARGV[0]
tag=ARGV[1]

if os == "linux"
    puts "linux"
    File.rename(file,file + "-" + os + "-" + tag)
end

if os == "osx"
    puts "osx"
    File.rename(file,file + "-" + os + "-" + tag)
end

if os == "windows"
    puts "windows"
    File.rename(file+".exe",file + "-" + os + "-" + tag + ".exe")
end
