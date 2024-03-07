// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include "entities_registration.h"
#include "interactor_registration.h"
#include "persistence_registration.h"
#include <QCoroQml>
#include <QGuiApplication>
#include <QQmlApplicationEngine>


int main(int argc, char *argv[])
{
    qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "1");
    qputenv("QT_LOGGING_RULES", "qt.qml.connections=false");
    qputenv("QT_QUICK_CONTROLS_CONF", ":/qtquickcontrols2.conf");
    qputenv("QML_COMPAT_RESOLVE_URLS_ON_ASSIGNMENT", "1");
    qputenv("QT_ENABLE_HIGHDPI_SCALING", "0");

    QGuiApplication app(argc, argv);
    app.setApplicationName("FrontEndsExample");
    app.setOrganizationName("frontendsexample");
    app.setOrganizationDomain("qleany.eu");

    new FrontEnds::Entities::EntitiesRegistration(&app);
    auto *persistenceRegistration = new FrontEnds::Persistence::PersistenceRegistration(&app);
    new FrontEnds::Interactor::InteractorRegistration(&app, persistenceRegistration->repositoryProvider());

    QCoro::Qml::registerTypes();

    QQmlApplicationEngine engine;
    const QUrl url(u"qrc:/qt/qml/Main/main.qml"_qs);
    QObject::connect(
        &engine, &QQmlApplicationEngine::objectCreated, &app,
        [url](QObject *obj, const QUrl &objUrl) {
            if (!obj && (url == objUrl))
                QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection);

    engine.addImportPath(QCoreApplication::applicationDirPath() + "/qml");
    engine.addImportPath(":/");
#if defined(MOCKS)
    engine.addImportPath(QCoreApplication::applicationDirPath() + "/mock_imports");
#endif

    engine.load(url);

    if (engine.rootObjects().isEmpty())
    {
        return -1;
    }

    return app.exec();
}