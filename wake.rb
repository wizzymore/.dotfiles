#!/usr/bin/env ruby
require 'socket'

DEVICES = {
  "windows" => "\x74\x56\x3c\x6f\xb8\x0d",
  "mini"    => "\x20\xa5\xcb\xd1\xa4\xff"
}

def send_wol(mac_bytes)
  # (6 octeți de \xff + MAC duplicat de 16 ori)
  packet = ("\xff" * 6) + (mac_bytes * 16)
  
  socket = UDPSocket.new
  socket.setsockopt(Socket::SOL_SOCKET, Socket::SO_BROADCAST, true)
  socket.send(packet, 0, '255.255.255.255', 9)
  socket.close
end

def main
  if ARGV.empty?
    puts "Eroare: Specificați dispozitivul. Utilizare: ruby wake.rb [windows|mini]"
    exit 1
  end

  target = ARGV[0].downcase

  if DEVICES.key?(target)
    puts "Se trimite pachetul de trezire catre: #{target}..."
    send_wol(DEVICES[target])
    puts "Pachet trimis cu succes!"
  else
    puts "Eroare: Dispozitiv necunoscut '#{target}'. Optiuni valide: windows, mini"
    exit 1
  end
end

main if __FILE__ == $0
