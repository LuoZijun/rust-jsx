
<React.Fragment>
    <App>
        <Page />
    </App>
    <App2>
        <Page />
        <Page />
        <Page />
    </App2>
</React.Fragment>

<App />
<App></App>
<App.subname></App.subname>
<App.subname.subname></App.subname.subname>

<App abc="abc" num={8} bool={true} js-expr={ [1, 2, 3] }></App>

<Page>
    Text
    <Text>abc</Text>
    { [1, 2, 3].map(function (v, idx) {
        return <Text key={idx}>{v}</Text>;
    }) }
</Page>
