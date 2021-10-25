from random import choice
import sys

LISTA_AEROPUERTOS = [
    'EZE',
    'AER',
    'GUA',
    'BRA',
    'SAO',
    'BAR',
    'MAD',
    'MIA',
    'JFK',
    'LGA',
    'BRC',
    'RHD',
    'AUS',
    'MEX',
    'LAX',
    'LAS',
    'HAJ',
    'GUI'
]

LISTA_AEROLINEA = [
    'AA',
    'GOL',
    'LAN',
    'CO',
    'AVIANCA',
    'UA',
    'CP',
    'DELTA'
]

PAQUETE = [
    'V',
    'P'
]

def random_result():
    salida = choice(LISTA_AEROPUERTOS)
    destino = choice(LISTA_AEROPUERTOS)
    while (salida == destino):
        destino = choice(LISTA_AEROPUERTOS)

    aerolinea = choice(LISTA_AEROLINEA)
    paquete = choice(PAQUETE)

    return f"{salida},{destino},{aerolinea},{paquete}"

def main():
    numero = sys.argv[1]
    N = 100 if len(sys.argv) <= 2 else int(sys.argv[2])

    with open(f"files/example-{numero}.csv", 'w+') as archivo:
        for i in range(N):
            archivo.write(random_result())
            archivo.write('\n')

main()