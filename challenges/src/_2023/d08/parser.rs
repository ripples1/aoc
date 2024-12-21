use nom::{
  branch::alt,
  bytes::complete::{tag, take_while},
  character::{
    complete::{alpha1, space0},
    streaming::multispace0,
  },
  combinator::{complete, opt},
  multi::many1,
  sequence::{delimited, separated_pair, terminated, tuple},
  IResult,
};

pub type NodeName<'a> = &'a str;
pub type NodeConnections<'a> = (NodeName<'a>, NodeName<'a>);
pub type Node<'a> = (NodeName<'a>, NodeConnections<'a>);

fn is_path_component(c: char) -> bool {
  c == 'L' || c == 'R'
}

fn path(s: &str) -> IResult<&str, &str> {
  terminated(take_while(is_path_component), multispace0)(s)
}

fn node_name(s: &str) -> IResult<&str, NodeName> {
  alpha1(s)
}

fn node_connections(s: &str) -> IResult<&str, NodeConnections> {
  delimited(
    tag("("),
    separated_pair(node_name, terminated(tag(","), space0), node_name),
    tag(")"),
  )(s)
}

fn node(s: &str) -> IResult<&str, Node> {
  terminated(
    separated_pair(
      node_name,
      delimited(space0, tag("="), space0),
      node_connections,
    ),
    opt(complete(multispace0)),
  )(s)
}

fn node_list(s: &str) -> IResult<&str, Vec<Node>> {
  many1(node)(s)
}

pub fn parser(
  s: &str,
) -> Result<(&str, Vec<Node>), nom::Err<nom::error::Error<&str>>> {
  let (_, result) = tuple((path, node_list))(s)?;
  Ok(result)
}
