// use libp2p::{identity, PeerId, Multiaddr};
// use libp2p::futures::StreamExt;
// use libp2p::swarm::{DummyBehaviour, Swarm, SwarmEvent};
// use libp2p::ping::{Ping, PingConfig};

use libp2p::{
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 密钥对的类型是 25519
    let new_key = identity::Keypair::generate_ed25519();
    /*
        使用密钥对的公钥(new_key.public())生产 PeerId
        在 libp2p 中，公钥不会用来直接验证 Peer 的身份，
        我们使用的是它的一个 hash 的版本，也就是这个 peer_id
     */
    let new_peer_id = PeerId::from(new_key.public());
    println!("New Peer ID is {:?}", new_peer_id);

    // 创建一个传输
    let transport = libp2p::development_transport(new_key).await?;

    // for create peer id: 创建一个空的网络行为，这个行为会关联到 swarm
    // let behaviour = DummyBehaviour::default();

    // for ping: 创建 ping 网络行为
    // let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    // for mdns
    let behaviour = Mdns::new(MdnsConfig::default()).await?;

    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    // 让 swarm 监听 0.0.0.0 地址，端口是 0，
    // 0.0.0.0 表示本地机器上所有的 ipv4 的地址
    // 端口 0 指随机选一个端口
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 
    /*
        本地向远程地址发出连接的代码
        远程地址是命令行输入的参数中取出的
        only for ping

     */
    // if let Some(remote_peer) = std::env::args().nth(1) {
    //     let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
    //     swarm.dial(remote_peer_multiaddr)?;
    //     println!("Dialed remote peer: {:?}", remote_peer);
    // }

    /*
        持续的轮询来检查事件
     */
    loop {
        match swarm.select_next_some().await {
            /*
                如果是 SwarmEvent::NewListenAddr 事件，
                即创建一个新的监听地址
             */
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on Local Address {:?}", address);
            }
            /*
                当本地节点发送 Ping 消息时，远程节点会返回 Pong，
                接收到 Pong 消息，会打印下边代码
                only for ping

             */
            // SwarmEvent::Behaviour(event) => {
            //     println!("Event received from peer is {:?}", event);
            // }
            SwarmEvent::Behaviour(
                // 这个事件，表示发现 peer 了
                MdnsEvent::Discovered(peers)) => {
                    for (peer, addr) in peers {
                        println!("discovered peer={}, addr={}", peer, addr);
                    }
            }
            SwarmEvent::Behaviour(
                // 这个事件，表示过期了
                MdnsEvent::Expired(expired)) => {
                    for (peer, addr) in expired {
                        println!("expired peer={}, addr={}", peer, addr);
                    }
            }
            _ => {}
        }
    }

}
