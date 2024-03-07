// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR GPL-3.0

#include "app_environment.h"
#include "entities_registration.h"
#include "import_qml_plugins.h"
#include "interactor_registration.h"
#include "persistence_registration.h"
#include <QCoroQml>
#include <QGuiApplication>
#include <QQmlApplicationEngine>

int main(int argc, char *argv[])
{
    set_qt_environment();
    QGuiApplication app(argc, argv);


    new Simple::Entities::EntitiesRegistration(&app);
    auto *persistenceRegistration = new Simple::Persistence::PersistenceRegistration(&app);
    new Simple::Interactor::InteractorRegistration(&app, persistenceRegistration->repositoryProvider());

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

    engine.load(url);

    if (engine.rootObjects().isEmpty())
    {
        return -1;
    }

    return app.exec();
}
