// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "presenter_export.h"
#include <QObject>

namespace Simple::Presenter
{
class SIMPLEEXAMPLE_PRESENTER_EXPORT SingleCar : public QObject
{
    Q_OBJECT
    Q_PROPERTY(int id READ id WRITE setId RESET resetId NOTIFY idChanged FINAL)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)

  public:
    explicit SingleCar(QObject *parent = nullptr);

    int id() const;
    void setId(int newId);
    void resetId();

    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

    QDateTime creationDate() const;
    void setCreationDate(const QDateTime &newCreationDate);

    QDateTime updateDate() const;
    void setUpdateDate(const QDateTime &newUpdateDate);

    QString content() const;
    void setContent(const QString &newContent);

  signals:

    void idChanged();

    void uuidChanged();

    void creationDateChanged();

    void updateDateChanged();

    void contentChanged();

  private:
    int m_id;

    QUuid m_uuid;

    QDateTime m_creationDate;

    QDateTime m_updateDate;

    QString m_content;
};

} // namespace Simple::Presenter