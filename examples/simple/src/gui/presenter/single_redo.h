// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_presenter_export.h"
#include <QAction>
#include <QObject>

namespace Simple::Presenter
{
class SIMPLE_EXAMPLE_PRESENTER_EXPORT SingleRedo : public QObject
{
    Q_OBJECT

  public:
    explicit SingleRedo(QObject *parent = nullptr);

    bool enabled() const;
    QString text() const;

  public slots:
    void redo();

  signals:
    void enabledChanged();
    void textChanged();

  private:
    QAction *m_action;
    bool m_enabled;
    QString m_text;
};

} // namespace Simple::Presenter