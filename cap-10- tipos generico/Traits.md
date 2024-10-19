# Traits: Definindo comportamento compartilhado

Um trait define a funcionalidade que um tipo particular tem e pode compartilhar com outros tipos. Podemos usar traits para definir comportamento compartilhado de forma abstrata. Podemos usar trait bounds para especificar que um tipo genérico pode ser qualquer tipo que tenha determinado comportamento.

obs : Traits são similares a interfaces em outras linguagens, mas com algumas diferenças importantes.

## Definindo um Trait

O comportamento de um tipo consiste nos métodos que podemos chamar naquele tipo. Tipos diferentes compartilham o mesmo comportamento se pudermos chamar os mesmos métodos em todos esses tipos. Definições de trait são uma maneira de agrupar assinaturas de métodos para definir um conjunto de comportamentos necessários para atingir algum propósito.

Por exemplo, digamos que temos várias structs que armazenam vários tipos e quantidades de texto: uma NewsArticlestruct que armazena uma notícia arquivada em um local específico e uma Tweetque pode ter, no máximo, 280 caracteres, juntamente com metadados que indicam se foi um novo tuíte, um retuíte ou uma resposta a outro tuíte.

Queremos fazer uma biblioteca agregadora de mídia chamada crate aggregatorque pode exibir resumos de dados que podem ser armazenados em uma instância NewsArticleou Tweet . Para fazer isso, precisamos de um resumo de cada tipo e solicitaremos esse resumo chamando um summarizemétodo em uma instância. A Listagem 10-12 mostra a definição de um Summarytrait público que expressa esse comportamento.


```rust 
pub trait Summary {
    fn summarize(&self) -> String;
}

```

Aqui, declaramos um trait usando a traitpalavra-chave e então o nome do trait, que é Summaryneste caso. Também declaramos o trait como pubpara que as caixas que dependem deste crate possam fazer uso deste trait também, como veremos em alguns exemplos. Dentro das chaves, declaramos as assinaturas de método que descrevem os comportamentos dos tipos que implementam este trait, que neste caso é fn summarize(&self) -> String.

Após a assinatura do método, em vez de fornecer uma implementação entre chaves, usamos um ponto e vírgula. Cada tipo que implementa esse trait deve fornecer seu próprio comportamento personalizado para o corpo do método. O compilador imporá que qualquer tipo que tenha o Summarytrait terá o método summarize definido com essa assinatura exatamente.

Uma característica pode ter vários métodos em seu corpo: as assinaturas dos métodos são listadas uma por linha, e cada linha termina com um ponto e vírgula.


## Implementando um Trait em um Tipo

Agora que definimos as assinaturas desejadas dos Summarymétodos do trait, podemos implementá-lo nos tipos em nosso agregador de mídia. A Listagem 10-13 mostra uma implementação do Summarytrait no NewsArticlestruct que usa o título, o autor e o local para criar o valor de retorno de summarize. Para o Tweetstruct, definimos summarizecomo o nome de usuário seguido pelo texto inteiro do tweet, assumindo que o conteúdo do tweet já esteja limitado a 280 caracteres.

Nome do arquivo: src/lib.rs

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

```

## Implementação padrão

Às vezes é útil ter um comportamento padrão para alguns ou todos os métodos em um trait em vez de exigir implementações para todos os métodos em cada tipo. Então, conforme implementamos o trait em um tipo específico, podemos manter ou substituir o comportamento padrão de cada método.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());


```


## resumo

Traits e limites de trait nos permitem escrever código que usa parâmetros de tipo genérico para reduzir duplicação, mas também especificam ao compilador que queremos que o tipo genérico tenha um comportamento particular. O compilador pode então usar as informações de limite de trait para verificar se todos os tipos concretos usados ​​com nosso código fornecem o comportamento correto. Em linguagens dinamicamente tipadas, obteríamos um erro em tempo de execução se chamássemos um método em um tipo que não definisse o método. Mas Rust move esses erros para o tempo de compilação, então somos forçados a corrigir os problemas antes mesmo que nosso código seja capaz de executar. Além disso, não precisamos escrever código que verifique o comportamento em tempo de execução porque já verificamos em tempo de compilação. Fazer isso melhora o desempenho sem ter que abrir mão da flexibilidade dos genéricos.